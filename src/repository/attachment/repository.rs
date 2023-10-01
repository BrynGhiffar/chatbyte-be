use sqlx::{Executor, Postgres, Pool};

use super::{AttachmentRepositoryModel, AttachmentFileType};

#[derive(Clone)]
pub struct AttachmentRepository {
    conn: Pool<Postgres>
}


impl AttachmentRepository {

    pub fn new(conn: Pool<Postgres>) -> Self {
        Self { conn }
    }

    pub async fn create_attachment(
        &self,
        name: &str,
        attachment: &[u8],
        attachment_file_type: AttachmentFileType
    ) -> Result<AttachmentRepositoryModel, String> {
        Self::create_attachment_with_executor(
            &self.conn, 
            name, 
            attachment, 
            attachment_file_type
        ).await
    }

    pub async fn link_attachment_direct_message(
        &self,
        attachment_id: i32,
        message_id: i32
    ) -> Result<bool, String> {
        Self::link_attachment_direct_message_with_executor(
            &self.conn, 
            attachment_id, 
            message_id
        ).await
    }

    pub async fn link_attachment_group_message(
        &self,
        attachment_id: i32,
        group_message_id: i32,
    ) -> Result<bool, String> {
        Self::link_attachment_group_message_with_executor(
            &self.conn,
            attachment_id, 
            group_message_id
        ).await
    }

    pub async fn create_attachment_with_executor<'a, T>(
        exec: T,
        name: &str,
        attachment: &[u8], 
        attachment_file_type: AttachmentFileType
    ) -> Result<AttachmentRepositoryModel, String> 
    where T: Executor<'a, Database = Postgres>
    {
        sqlx::query_as::<_, AttachmentRepositoryModel>("
                INSERT INTO PUBLIC.ATTACHMENT(NAME, CONTENT, FILE_TYPE)
                VALUES ($1, $2, $3) RETURNING *
            ")
            .bind(name)
            .bind(attachment)
            .bind(attachment_file_type.to_string())
            .fetch_one(exec)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn link_attachment_direct_message_with_executor<'a, T>(
        exec: T,
        attachment_id: i32,
        message_id: i32,
    ) -> Result<bool, String> 
    where T: Executor<'a, Database = Postgres>
    {
        sqlx::query("
                INSERT INTO PUBLIC.ATTACHMENT_MESSAGE(ATTACHMENT_ID, DIRECT_MESSAGE_ID)
                VALUES ($1, $2)
            ")
            .bind(attachment_id)
            .bind(message_id)
            .execute(exec)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn link_attachment_group_message_with_executor<'a, T>(
        exec:T,
        attachment_id: i32,
        group_message_id: i32,
    ) -> Result<bool, String> 
    where T: Executor<'a, Database = Postgres>
    {
        sqlx::query("
                INSERT INTO PUBLIC.ATTACHMENT_MESSAGE(ATTACHMENT_ID, GROUP_MESSAGE_ID)
                VALUES ($1, $2)
            ")
            .bind(attachment_id)
            .bind(group_message_id)
            .execute(exec)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }
}