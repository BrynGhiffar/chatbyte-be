use std::{collections::HashMap, str::FromStr};

use crate::{
    websocket::message::{AppMessage, AppRx, SessionMessage, SessionTx},
    repository::{message::MessageRepository, entities::message, group::{GroupRepository, GroupMessage}},
};
use serde::{Deserialize, Serialize};
use serde_json::Error;

use super::session::SessionFactory;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsRequest {
    #[serde(rename = "SEND_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    SendMessage {
        receiver_uid: i32,
        message: String,
    },
    #[serde(rename = "READ_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    ReadMessage {
        receiver_uid: i32,
        sender_uid: i32
    },
    #[serde(rename = "SEND_GROUP_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    SendGroupMessage {
        group_id: i32,
        message: String
    }
}

impl FromStr for WsRequest {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl ToString for WsRequest {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum WsResponse {
    #[serde(rename = "MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    MessageNotification {
        id: i32,
        sender_uid: i32,
        receiver_uid: i32,
        content: String,
        is_user: bool,
        sent_at: String,
        receiver_read: bool
    },

    #[serde(rename = "GROUP_MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    GroupMessageNotification {
        id: i32,
        sender_id: i32,
        username: String,
        group_id: i32,
        content: String,
        sent_at: String
    },

    #[serde(rename = "READ_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    ReadNotification {
        sender_uid: i32,
        receiver_uid: i32
    },

    #[serde(rename = "ERROR_NOTIFICATION")]
    ErrorNotification { message: String },
}

impl ToString for WsResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl WsResponse {
    fn from_group_message(message: GroupMessage) -> Self {
        Self::GroupMessageNotification { 
            id: message.id, 
            sender_id: message.sender_id,
            username: message.username, 
            group_id: message.group_id, 
            content: message.content, 
            sent_at: message.sent_at.format("%H:%M").to_string()
        }
    }
}

pub struct WsServer {
    user_storage: HashMap<i32, SessionTx>,
    app_rx: AppRx,
    message_repository: MessageRepository,
    group_repository: GroupRepository
}

impl WsServer {
    pub fn new(
        message_repository: MessageRepository,
        group_repository: GroupRepository
    ) -> (Self, SessionFactory) {
        let (app_tx, app_rx) = AppMessage::channel();

        let user_storage = HashMap::new();
        let message_repository = message_repository;
        let ws_server = Self {
            user_storage,
            app_rx,
            message_repository,
            group_repository
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

    fn send_session_message(&self, user_id: i32, message: String) {
        let Some(sess_tx) = self.user_storage.get(&user_id) else { return; };
        sess_tx
            .send(SessionMessage::Message(message))
            .unwrap();
    }

    fn send_session_error(&self, user_id: i32, message: String) {
        let Some(sess_tx) = self.user_storage.get(&user_id) else { return; };
        sess_tx
            .send(SessionMessage::Message(WsResponse::ErrorNotification { message }.to_string()))
            .unwrap();
    }

    fn send_new_message_notification(&self, user_id: i32, msg: message::Model) {

        let message = WsResponse::MessageNotification {
            id: msg.id,
            sender_uid: msg.sender_id,
            receiver_uid: msg.receiver_id,
            is_user: user_id == msg.sender_id,
            content: msg.content,
            sent_at: msg.sent_at.format("%H:%M").to_string(),
            receiver_read: msg.read
        };
        self.send_session_message(user_id, message.to_string());
    }

    fn handle_read_message(&self, receiver_uid: i32, sender_uid: i32) {
        let message = WsResponse::ReadNotification { sender_uid, receiver_uid };
        self.send_session_message(sender_uid, message.to_string());
    }

    async fn handle_user_send_message(&self, sender_uid: i32, receiver_uid: i32, msg: String) {
        let res = self
            .message_repository
            .insert_message(receiver_uid, sender_uid, msg.clone())
            .await;
        let msg = match res {
            Ok(msg) => msg,
            Err(e) => {
                self.send_session_error(sender_uid, e.to_string());
                return;
            }
        };
        self.send_new_message_notification(sender_uid, msg.clone());
        self.send_new_message_notification(receiver_uid, msg);


    }

    async fn handle_send_group_message(&self, sender_uid: i32, group_id: i32, message: String) {
        let result = self.group_repository.find_group_members(group_id).await;
        let member_ids = match result {
            Ok(ids) => ids,
            _ => return
        };
        if !member_ids.contains(&sender_uid) { return; }
        let result = self.group_repository.create_group_message(group_id, sender_uid, message).await;
        let message = match result {
            Ok(m) => WsResponse::from_group_message(m).to_string(),
            _ => return
        };
        for mid in member_ids.iter() {
            self.send_session_message(*mid, message.clone());
        }
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
            },
            WsRequest::ReadMessage { receiver_uid, sender_uid } => self.handle_read_message(receiver_uid, sender_uid),
            WsRequest::SendGroupMessage { group_id, message } => self.handle_send_group_message(session_id, group_id, message).await
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
