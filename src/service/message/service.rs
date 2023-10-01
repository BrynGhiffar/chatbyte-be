use std::io::Cursor;

use bindet::FileType;
use sqlx::{Pool, Postgres, Acquire, Executor};

use crate::repository::{MessageRepository, GroupRepository, AttachmentFileType, AttachmentRepository};

use super::{CreateGroupMessageModel, GroupMessageModel, AttachmentModel, CreateDirectMessageModel, DirectMessageModel, CreateAttachmentModel};

#[derive(Clone)]
pub struct MessageService {
    conn: Pool<Postgres>,
    message_repository: MessageRepository,
    group_repository: GroupRepository,
}


impl MessageService {

    pub fn new(conn: Pool<Postgres>) -> Self {
        let message_repository = MessageRepository::new(conn.clone());
        let group_repository = GroupRepository::new(conn.clone());
        Self { conn, message_repository, group_repository }
    }
    
    pub async fn upload_message_attachments<'a, T>(
        exec: T,
        create_attachment_models: CreateAttachmentModel,
    ) -> Result<AttachmentModel, String>
        where T: Executor<'a, Database = Postgres>
    {
        let name = &create_attachment_models.name;
        let attachment = &create_attachment_models.attachment;
        let att_type = match detect_file_type(&attachment) {
            Ok(r) => r,
            Err(e) => return Err(e)
        };
        let att = AttachmentRepository::
            create_attachment_with_executor(exec, name, attachment, att_type)
            .await
            .map_err(|e| e.to_string())?
            ;
        Ok(att.into())
    }

    pub async fn create_group_message(&self, message: CreateGroupMessageModel) -> Result<GroupMessageModel, String> {
        log::info!("Creating group message");
        let CreateGroupMessageModel { group_id, sender_id, content, attachment } = message;
        let mut tx = self.conn.begin().await.map_err(|e| e.to_string())?;
        let conn = tx.acquire().await.map_err(|e| e.to_string())?;
        let message = GroupRepository::
            create_group_message_with_executor(conn, group_id, sender_id, content)
            .await
            .map_err(|e| e.to_string())?;
        log::info!("Created group message, adding attachments...");
        let mut attachments = Vec::<AttachmentModel>::new();
        for att in attachment {
            let conn = tx.acquire().await.map_err(|e| e.to_string())?;
            let att_res = Self::upload_message_attachments(conn, att).await?;
            let conn = tx.acquire().await.map_err(|e| e.to_string())?;
            let succ = AttachmentRepository::
                link_attachment_group_message_with_executor(
                    conn, 
                    att_res.id, 
                    message.id
                ).await.map_err(|e| e.to_string())?;
            if !succ {
                return Err("Failed linking attachment with group message".to_string());
            }
            attachments.push(att_res);
        }
        log::info!("Inserted message data into database");
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(GroupMessageModel { 
            id: message.id, 
            sender_id: message.sender_id, 
            username: message.username.clone(), 
            group_id: message.group_id, 
            content: message.content.clone(), 
            sent_at: message.sent_at, 
            edited: message.edited, 
            deleted: message.deleted, 
            attachments 
        })
    }

    pub async fn create_direct_message(&self, message: CreateDirectMessageModel) -> Result<DirectMessageModel, String> {
        log::info!("Created direct message");
        let CreateDirectMessageModel { receiver_id, sender_id, content, attachment } = message;
        let mut tx = self.conn.begin().await.map_err(|e| e.to_string())?;
        let exec = tx.acquire().await.map_err(|e| e.to_string())?;
        let message = MessageRepository::
            insert_message_with_executor(exec, receiver_id, sender_id, content)
            .await
            .map_err(|e| e.to_string())?;
        log::info!("Created direct message. Adding attachments...");
        let mut attachments = Vec::<AttachmentModel>::new();
        for att in attachment {
            let exec = tx.acquire().await.map_err(|e| e.to_string())?;
            let att_res = Self::upload_message_attachments(exec, att).await?;
            let exec = tx.acquire().await.map_err(|e| e.to_string())?;
            let succ = AttachmentRepository::
                link_attachment_direct_message_with_executor(
                    exec, 
                    att_res.id, 
                    message.id
                ).await
                .map_err(|e| e.to_string())?;
            if !succ {
                return Err("Failed linking attachment with direct message".to_string());
            }
            attachments.push(att_res);
        }
        log::info!("Inserted message into database");
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(DirectMessageModel { 
            id: message.id, 
            sent_at: message.sent_at, 
            sender_id: message.sender_id, 
            receiver_id: message.receiver_id, 
            content: message.content, 
            read: message.read, 
            edited: message.edited, 
            deleted: message.deleted, 
            attachments 
        })
    }
}

pub fn detect_file_type(content: &[u8]) -> Result<AttachmentFileType, String> {
    use AttachmentFileType::*;
    let cursor = Cursor::new(content);
    let detect = bindet::detect(cursor);
    
    match detect {
        Ok(Some(ft)) => {
            if ft.likely_to_be.contains(&FileType::Png) {
                Ok(Png)
            } else if ft.likely_to_be.contains(&FileType::Jpg) {
                Ok(Jpeg)
            } else {
                Err("File type not supported".to_string())
            }
        },
        Ok(None) => Err("File type not found".to_string()),
        Err(e) => Err(e.to_string())
    }
}