use std::time::Duration;

use crate::middleware::verify_token;
use crate::websocket::message::AppMessage;
use crate::websocket::message::AppTx;
use crate::websocket::message::SessionMessage;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::Error;
use futures_util::FutureExt;
use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::SplitSink;
use merge_streams::MergeStreams;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

enum WebSocketMessage {
    Text(String),
    Close
}


#[derive(Clone)]
pub struct SessionFactory {
    pub app_tx: AppTx,
}

impl SessionFactory {
    pub fn create_session(&self, token: String, user_id: i32) -> Session {
        // create a session
        // add session into the database

        let session_id = user_id;
        Session {
            session_id,
            token,
            app_tx: self.app_tx.clone(),
        }
    }
}

type AxumWsMessageRecv = Option<Result<Message, Error>>;

enum SessionSource {
    WebSocketText(String),
    WebSocketClose,
    WebSocketError(Error),
    SessionMessage(SessionMessage),
    TokenChecker(bool),
}

pub struct Session {
    session_id: i32,
    token: String,
    app_tx: AppTx,
}

impl Session {
    pub async fn handle_session_message(ws: &mut SplitSink<WebSocket, Message>, msg: String) {
        ws.send(Message::Text(msg)).await.unwrap();
    }

    pub async fn handle_websocket_message(&self, msg: String) {
        self.app_tx.send(AppMessage::Message {
            session_id: self.session_id,
            message: msg.to_string(),
        })
        .unwrap();
    }

    pub async fn run(self, ws: WebSocket) {
        let (mut ws_tx, mut ws_rx) = ws.split();
        let (session_tx, mut session_rx) = SessionMessage::channel();
        let (token_checker, mut checker_rx) = self.spawn_token_checker(self.session_id);
        self.app_tx
            .send(AppMessage::Connect {
                session_id: self.session_id,
                sess_tx: session_tx,
            })
            .expect("This should be okay");
        loop {
            
            let ws_source = ws_rx
                .next()
                .into_stream()
                .filter_map(|s| async move { s })
                .filter_map(|r| async move {
                    match r {
                        Ok(Message::Text(text)) => Some(SessionSource::WebSocketText(text)),
                        Ok(Message::Close(_)) => Some(SessionSource::WebSocketClose),
                        Ok(_) => None,
                        Err(e) => Some(SessionSource::WebSocketError(e)),
                    }
                });
            let session_message_source = session_rx
                .recv()
                .into_stream()
                .filter_map(|s| async move { s })
                .map(SessionSource::SessionMessage);
            let token_checker_source = checker_rx
                .recv()
                .into_stream()
                .filter_map(|s| async move { s })
                .map(SessionSource::TokenChecker);
            let mut source = (ws_source, session_message_source, token_checker_source).merge();

            let Some(msg) = source.next().await else {
                continue;
            };

            match msg {
                SessionSource::SessionMessage(SessionMessage::Message(text)) => {
                    Self::handle_session_message(&mut ws_tx, text).await;
                },
                SessionSource::WebSocketText(msg) => { 
                    self.handle_websocket_message(msg).await;
                },
                
                // Errors
                SessionSource::WebSocketError(e) => {
                    log::info!("{e}");
                },
                
                // close connections

                // from token checker
                SessionSource::TokenChecker(_) => {
                    self.app_tx.send(AppMessage::Disconnect { session_id: self.session_id, }).unwrap();
                    ws_tx.send(Message::Close(None)).await.unwrap();
                    break;
                },
                // from server
                SessionSource::SessionMessage(SessionMessage::CloseConnection) => {
                    token_checker.abort();
                    ws_tx.send(Message::Close(None)).await.unwrap();
                    break;
                },
                // from client
                SessionSource::WebSocketClose => {
                    token_checker.abort();
                    self.app_tx.send(AppMessage::Disconnect { session_id: self.session_id, }).unwrap();
                    break;
                }
            };
        }
    }

    pub fn spawn_token_checker(&self, _session_id: i32) -> (JoinHandle<()>, mpsc::UnboundedReceiver<bool>) {
        let (ch_tx, ch_rx) = mpsc::unbounded_channel::<bool>();
        let token = self.token.clone();
        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                // log::info!("Checking token for session: {session_id}");
                let valid = verify_token(token.clone());
                if let Err(_) = valid {
                    ch_tx.send(true).unwrap();
                    break;
                }
            }
            ()
        });
        (handle, ch_rx)
    }
}