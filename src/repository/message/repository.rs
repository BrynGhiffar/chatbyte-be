use chrono::Local;
use sqlx::{Pool, Postgres};

use super::{Message, GET_MESSAGE_BETWEEN_USER_STMT, UPDATE_MESSAGE_READ_STMT, ConversationRecentMessages, CREATE_MESSAGE_STMT, GET_RECENT_MESSAGE_STMT, DELETE_MESSAGE_STMT, FIND_MESSAGE_BY_ID_STMT, EDIT_MESSAGE_BY_ID_STMT};

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
    ) -> Result<Vec<Message>, String> {
        sqlx::query_as::<_, Message>(GET_MESSAGE_BETWEEN_USER_STMT)
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
    ) -> Result<Vec<ConversationRecentMessages>, String> {
        sqlx::query_as::<_,ConversationRecentMessages>(GET_RECENT_MESSAGE_STMT)
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
    ) -> Result<Message, String> {
        let sent_at = Local::now().naive_local();
        sqlx::query_as::<_, Message>(CREATE_MESSAGE_STMT)
            .bind(sender_uid)
            .bind(receiver_uid)
            .bind(content)
            .bind(sent_at)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())

    }

    pub async fn find_message_by_id(
        &self,
        message_id: i32,
    ) -> Result<Option<Message>, String> {
        sqlx::query_as::<_, Message>(FIND_MESSAGE_BY_ID_STMT)
            .bind(message_id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_message(
        &self,
        message_id: i32
    ) -> Result<bool, String> {
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
        content: String
    ) -> Result<Message, String> {
        sqlx::query_as::<_, Message>(EDIT_MESSAGE_BY_ID_STMT)
            .bind(message_id)
            .bind(content)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}

