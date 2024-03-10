use chrono::Local;
use sqlx::Executor;
use sqlx::Pool;
use sqlx::Postgres;

use super::ConversationRecentMessageRepositoryModel;
use super::MessageRepositoryModel;
use super::CREATE_MESSAGE_STMT;
use super::DELETE_MESSAGE_STMT;
use super::EDIT_MESSAGE_BY_ID_STMT;
use super::FIND_MESSAGE_BY_ID_STMT;
use super::GET_MESSAGE_BETWEEN_USER_STMT;
use super::GET_RECENT_MESSAGE_STMT;
use super::UPDATE_MESSAGE_READ_STMT;

#[derive(Clone)]
pub struct MessageRepository {
    conn: Pool<Postgres>,
}

impl MessageRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        MessageRepository { conn }
    }

    pub async fn get_message_between_users(
        &self,
        user1_uid: i32,
        user2_uid: i32,
    ) -> Result<Vec<MessageRepositoryModel>, String> {
        sqlx::query_as::<_, MessageRepositoryModel>(GET_MESSAGE_BETWEEN_USER_STMT)
            .bind(user1_uid)
            .bind(user2_uid)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_message_read(&self, to_user: i32, from_user: i32) -> Result<(), String> {
        sqlx::query(UPDATE_MESSAGE_READ_STMT)
            .bind(to_user)
            .bind(from_user)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    pub async fn get_recent_messages(
        &self,
        user_id: i32,
    ) -> Result<Vec<ConversationRecentMessageRepositoryModel>, String> {
        sqlx::query_as::<_, ConversationRecentMessageRepositoryModel>(GET_RECENT_MESSAGE_STMT)
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn insert_message(
        &self,
        receiver_uid: i32,
        sender_uid: i32,
        content: String,
    ) -> Result<MessageRepositoryModel, String> {
        Self::insert_message_with_executor(&self.conn, receiver_uid, sender_uid, content).await
    }

    pub async fn insert_message_with_executor<'a, T>(
        exec: T,
        receiver_uid: i32,
        sender_uid: i32,
        content: String,
    ) -> Result<MessageRepositoryModel, String>
    where
        T: Executor<'a, Database = Postgres>,
    {
        let sent_at = Local::now().naive_local();
        sqlx::query_as::<_, MessageRepositoryModel>(CREATE_MESSAGE_STMT)
            .bind(sender_uid)
            .bind(receiver_uid)
            .bind(content)
            .bind(sent_at)
            .fetch_one(exec)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_message_by_id(
        &self,
        message_id: i32,
    ) -> Result<Option<MessageRepositoryModel>, String> {
        sqlx::query_as::<_, MessageRepositoryModel>(FIND_MESSAGE_BY_ID_STMT)
            .bind(message_id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_message(&self, message_id: i32) -> Result<bool, String> {
        sqlx::query(DELETE_MESSAGE_STMT)
            .bind(message_id)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn edit_message_by_id(
        &self,
        message_id: i32,
        content: String,
    ) -> Result<MessageRepositoryModel, String> {
        sqlx::query_as::<_, MessageRepositoryModel>(EDIT_MESSAGE_BY_ID_STMT)
            .bind(message_id)
            .bind(content)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}
