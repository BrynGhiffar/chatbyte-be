use std::time::Duration;

use crate::{
    message::application::{AppMessage, AppTx, SessionMessage},
    middleware::verify_token,
};
use actix_ws::{Message, MessageStream, ProtocolError};
use futures_util::{FutureExt, StreamExt};
use merge_streams::MergeStreams;
use tokio::{
    sync::mpsc,
    task::{spawn_local, JoinHandle},
};

#[derive(Clone)]
pub struct SessionFactory {
    pub app_tx: AppTx,
}

impl SessionFactory {
    pub fn create_session(&self, token: String, user_id: i32, ws_tx: actix_ws::Session) -> Session {
        // create a session
        // add session into the database

        let session_id = user_id;
        Session {
            session_id,
            token,
            app_tx: self.app_tx.clone(),
            ws_tx,
        }
    }
}

enum SessionSource {
    WebSocketConnection(Option<Result<Message, ProtocolError>>),
    SessionMessage(Option<SessionMessage>),
    TokenChecker(bool),
}

pub struct Session {
    session_id: i32,
    token: String,
    app_tx: AppTx,
    ws_tx: actix_ws::Session,
}

impl Session {
    pub async fn handle_session_message(&mut self, msg: Option<SessionMessage>) -> bool {
        let Some(msg) = msg else { return false; };
        match msg {
            SessionMessage::Message(message) => self.ws_tx.text(message).await.unwrap(),
            SessionMessage::CloseConnection => return true,
        };
        return false;
    }

    pub async fn handle_websocket_message(&self, msg: Option<Result<Message, ProtocolError>>) {
        let Some(msg) = msg else { return; };
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };
        let res = match msg {
            Message::Text(msg) => self.app_tx.send(AppMessage::Message {
                session_id: self.session_id,
                message: msg.to_string(),
            }),
            Message::Close(_) => self.app_tx.send(AppMessage::Disconnect {
                session_id: self.session_id,
            }),
            _ => Ok(()),
        };
        res.unwrap()
    }

    pub async fn run(mut self, mut ws_rx: MessageStream) {
        let (session_tx, mut session_rx) = SessionMessage::channel();
        let (token_checker, mut checker_rx) = self.spawn_token_checker();
        self.app_tx
            .send(AppMessage::Connect {
                session_id: self.session_id,
                sess_tx: session_tx,
            })
            .unwrap();

        loop {
            let ws_source = ws_rx
                .recv()
                .into_stream()
                .map(SessionSource::WebSocketConnection);
            let session_message_source = session_rx
                .recv()
                .into_stream()
                .map(SessionSource::SessionMessage);
            let token_checker_source = checker_rx
                .recv()
                .into_stream()
                .filter_map(|s| async move { s })
                .map(SessionSource::TokenChecker);
            let mut source = (ws_source, session_message_source, token_checker_source).merge();

            let Some(msg) = source.next().await else { continue; };

            match msg {
                SessionSource::SessionMessage(msg) => {
                    let close = self.handle_session_message(msg).await;
                    if close {
                        break;
                    }
                }
                SessionSource::WebSocketConnection(msg) => self.handle_websocket_message(msg).await,
                SessionSource::TokenChecker(close) => {
                    if close {
                        break;
                    }
                }
            };
        }
        self.app_tx.send(AppMessage::Disconnect { session_id: self.session_id }).unwrap();
        token_checker.abort();
        self.ws_tx.close(None).await.unwrap();
    }

    pub fn spawn_token_checker(&self) -> (JoinHandle<()>, mpsc::UnboundedReceiver<bool>) {
        let (ch_tx, ch_rx) = mpsc::unbounded_channel::<bool>();
        let token = self.token.clone();
        let handle = spawn_local(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
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
