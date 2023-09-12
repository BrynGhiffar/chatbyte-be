use std::{collections::HashMap, str::FromStr};

use crate::{
    message::application::{AppMessage, AppRx, SessionMessage, SessionTx},
    repository::message_repository::MessageRepository,
};
use serde::{Deserialize, Serialize};
use serde_json::Error;

use super::session::SessionFactory;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum WsRequest {
    #[serde(rename = "SEND_MESSAGE")]
    SendMessage {
        #[serde(rename = "receiverUid")]
        receiver_uid: i32,
        message: String,
    },
}

impl FromStr for WsRequest {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum WsResponse {
    #[serde(rename = "MESSAGE_NOTIFICATION")]
    MessageNotification {
        id: i32,
        #[serde(rename = "senderUid")]
        sender_uid: i32,
        #[serde(rename = "receiverUid")]
        receiver_uid: i32,
        content: String,
        #[serde(rename = "isUser")]
        is_user: bool,
        #[serde(rename = "sentAt")]
        sent_at: String,
    },

    #[serde(rename = "ERROR_NOTIFICATION")]
    ErrorNotification { message: String },
}

impl ToString for WsResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct WsServer {
    user_storage: HashMap<i32, SessionTx>,
    app_rx: AppRx,
    message_repository: MessageRepository,
}

impl WsServer {
    pub fn new(message_repository: MessageRepository) -> (Self, SessionFactory) {
        let (app_tx, app_rx) = AppMessage::channel();

        let user_storage = HashMap::new();
        let message_repository = message_repository;
        let ws_server = Self {
            user_storage,
            app_rx,
            message_repository,
        };
        let session_factory = SessionFactory { app_tx };
        (ws_server, session_factory)
    }

    async fn session_up(&mut self, session_id: i32, sess_tx: SessionTx) {
        let prev = self.user_storage.insert(session_id, sess_tx);
        if let Some(session) = prev {
            let _ = session.send(SessionMessage::CloseConnection);
        }
    }

    async fn handle_user_send_message(&self, sender_uid: i32, receiver_uid: i32, msg: String) {
        let res = self
            .message_repository
            .insert_message(receiver_uid, sender_uid, msg.clone())
            .await;
        let Some(sess_tx) = self.user_storage.get(&receiver_uid) else { return; };
        let msg = match res {
            Ok(msg) => msg,
            Err(e) => {
                sess_tx
                    .send(SessionMessage::Message(
                        WsResponse::ErrorNotification {
                            message: e.to_string(),
                        }
                        .to_string(),
                    ))
                    .unwrap();
                return;
            }
        };
        let res = WsResponse::MessageNotification {
            id: msg.id,
            sender_uid,
            receiver_uid,
            is_user: true,
            content: msg.content,
            sent_at: msg.sent_at.format("%H:%M").to_string(),
        };
        sess_tx
            .send(SessionMessage::Message(res.to_string()))
            .unwrap();
    }

    async fn session_message(&mut self, session_id: i32, msg: String) {
        let Ok(message) = WsRequest::from_str(&msg) else { return; };
        match message {
            WsRequest::SendMessage {
                receiver_uid,
                message,
            } => {
                self.handle_user_send_message(session_id, receiver_uid, message)
                    .await
            }
        };
    }

    async fn session_down(&mut self, session_id: i32) {
        let sess = self.user_storage.remove(&session_id);
        if let Some(sess) = sess {
            let _ = sess.send(SessionMessage::CloseConnection);
        }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        while let Some(msg) = self.app_rx.recv().await {
            match msg {
                AppMessage::Connect {
                    session_id,
                    sess_tx,
                } => self.session_up(session_id, sess_tx).await,
                AppMessage::Message {
                    session_id,
                    message,
                } => self.session_message(session_id, message).await,
                AppMessage::Disconnect { session_id } => self.session_down(session_id).await,
            }
        }
        Ok(())
    }
}
