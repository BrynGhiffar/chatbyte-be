use chrono::NaiveDateTime;
use serde::Serialize;

use crate::repository::AttachmentFileType;
use crate::repository::AttachmentRepositoryModel;
use crate::repository::GroupMessageRepositoryModel;
use crate::repository::MessageRepositoryModel;

pub struct CreateGroupMessageModel {
    pub group_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub attachment: Vec<CreateAttachmentModel>,
}

#[derive(Clone)]
pub struct CreateAttachmentModel {
    pub name: String,
    pub attachment: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMessageModel {
    pub id: i32,
    pub sender_id: i32,
    pub username: String,
    pub group_id: i32,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub edited: bool,
    pub deleted: bool,
    pub attachments: Vec<AttachmentModel>,
}

impl GroupMessageModel {
    pub fn combine(
        GroupMessageRepositoryModel { 
            id, 
            sender_id, 
            username, 
            group_id, 
            content, 
            sent_at, 
            edited, 
            deleted 
        }: GroupMessageRepositoryModel,
        attachments: Vec<AttachmentRepositoryModel>
    ) -> Self {
        Self { 
            id, 
            sender_id, 
            username, 
            group_id, 
            content, 
            sent_at, 
            edited, 
            deleted, 
            attachments: attachments
            .iter()
            .map(|at| at.clone().into())
            .collect()
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentModel {
    pub id: i32,
    pub name: String,
    pub file_type: AttachmentFileType,
}

impl From<AttachmentRepositoryModel> for AttachmentModel {
    fn from(value: AttachmentRepositoryModel) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            file_type: value.file_type,
        }
    }
}

#[derive(Clone)]
pub struct CreateDirectMessageModel {
    pub receiver_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub attachment: Vec<CreateAttachmentModel>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageModel {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub read: bool,
    pub edited: bool,
    pub deleted: bool,
    pub attachments: Vec<AttachmentModel>,
}

impl DirectMessageModel {
    pub fn combine(
        MessageRepositoryModel { 
            id, 
            sent_at, 
            sender_id, 
            receiver_id, 
            content, 
            read, 
            edited, 
            deleted 
        }: MessageRepositoryModel,
        attachments: Vec<AttachmentRepositoryModel>
    ) -> Self {
        Self { 
            id, 
            sent_at, 
            sender_id, 
            receiver_id, 
            content, 
            read, 
            edited, 
            deleted, 
            attachments: attachments
                .iter()
                .map(|at| at.clone().into())
                .collect()
        }
    }
}