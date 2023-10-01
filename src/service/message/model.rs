use chrono::NaiveDateTime;

use crate::repository::{AttachmentFileType, AttachmentRepositoryModel};


pub struct CreateGroupMessageModel {
    pub group_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub attachment: Vec<CreateAttachmentModel>
}

#[derive(Clone)]
pub struct CreateAttachmentModel {
    pub name: String,
    pub attachment: Vec<u8>,
}

pub struct GroupMessageModel {
    pub id: i32,
    pub sender_id: i32,
    pub username: String,
    pub group_id: i32,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub edited: bool,
    pub deleted: bool,
    pub attachments: Vec<AttachmentModel>
}

#[derive(Clone)]
pub struct AttachmentModel {
    pub id: i32,
    pub name: String,
    pub file_type: AttachmentFileType
}

impl From<AttachmentRepositoryModel> for AttachmentModel {
    fn from(value: AttachmentRepositoryModel) -> Self {
        Self { id: value.id, name: value.name.clone(), file_type: value.file_type }
    }
}

#[derive(Clone)]
pub struct CreateDirectMessageModel {
    pub receiver_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub attachment: Vec<CreateAttachmentModel>
}

#[derive(Clone)]
pub struct DirectMessageModel {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub read: bool,
    pub edited: bool,
    pub deleted: bool,
    pub attachments: Vec<AttachmentModel>
}