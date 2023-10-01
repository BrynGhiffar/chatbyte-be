use std::str::FromStr;

use base64::{engine::general_purpose, Engine};
use serde::{ Serialize, Deserialize };
use serde_json::Error;

use crate::{repository::AttachmentFileType, service::GroupMessageModel};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttachment {
    pub name: String,
    pub content_base64: String
}

impl MessageAttachment {
    pub fn content_as_bytes(&self) -> Result<Vec<u8>, String> {
        general_purpose::STANDARD.decode(&self.content_base64)
            .map_err(|e| e.to_string())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsRequest {
    
    #[serde(rename = "SEND_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    SendMessage { 
        receiver_uid: i32, 
        message: String,
        attachments: Vec<MessageAttachment>
    },
    
    #[serde(rename = "SEND_GROUP_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    SendGroupMessage { 
        group_id: i32, 
        message: String,
        attachments: Vec<MessageAttachment>
    },
    
    #[serde(rename = "READ_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    ReadMessage { receiver_uid: i32, sender_uid: i32 },
    
    #[serde(rename = "DELETE_DIRECT_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    DeleteDirectMessage { message_id: i32, },
    
    #[serde(rename = "DELETE_GROUP_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    DeleteGroupMessage { message_id: i32 },
    
    #[serde(rename = "EDIT_DIRECT_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    EditDirectMessage { message_id: i32, edited_content: String },

    #[serde(rename = "EDIT_GROUP_MESSAGE")]
    #[serde(rename_all = "camelCase")]
    EditGroupMessage { message_id: i32, edited_content: String }
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

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageNotificationAttachment {
    pub id: i32,
    pub file_type: AttachmentFileType
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WsResponse {
    #[serde(rename = "MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    MessageNotification {
        id: i32,
        sender_uid: i32,
        receiver_uid: i32,
        content: String,
        is_user: bool,
        sent_at: String,
        receiver_read: bool,
        attachments: Vec<MessageNotificationAttachment>
    },

    #[serde(rename = "GROUP_MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    GroupMessageNotification {
        id: i32,
        sender_id: i32,
        username: String,
        group_id: i32,
        content: String,
        sent_at: String,
        attachments: Vec<MessageNotificationAttachment>
    },

    #[serde(rename = "READ_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    ReadNotification { sender_uid: i32, receiver_uid: i32 },

    #[serde(rename = "ERROR_NOTIFICATION")]
    ErrorNotification { message: String },

    #[serde(rename = "DELETE_DIRECT_MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    DeleteMessageNotification { contact_id: i32, message_id: i32 },

    #[serde(rename = "DELETE_GROUP_MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    DeleteGroupMessageNotification { group_id: i32, message_id: i32 },

    #[serde(rename = "UPDATE_DIRECT_MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    UpdateDirectMessageNotification { contact_id: i32, message_id: i32, content: String },

    #[serde(rename = "UPDATE_GROUP_MESSAGE_NOTIFICATION")]
    #[serde(rename_all = "camelCase")]
    UpdateGroupMessageNotification { group_id: i32, message_id: i32, content: String }
}

impl ToString for WsResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl WsResponse {
    pub fn from_group_message(message: GroupMessageModel) -> Self {
        Self::GroupMessageNotification {
            id: message.id,
            sender_id: message.sender_id,
            username: message.username,
            group_id: message.group_id,
            content: message.content,
            sent_at: message.sent_at.format("%H:%M").to_string(),
            attachments: message.attachments
            .iter()
            .map(|at| MessageNotificationAttachment {
                id: at.id,
                file_type: at.file_type.clone()
            }).collect()
        }
    }
}
