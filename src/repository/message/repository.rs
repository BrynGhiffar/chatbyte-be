use chrono::Local;
use sqlx::{Pool, Postgres};

use super::{Message, GET_MESSAGE_BETWEEN_USER_STMT, UPDATE_MESSAGE_READ_STMT, ConversationRecentMessages, CREATE_MESSAGE_STMT};

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
        sqlx::query_as(GET_MESSAGE_BETWEEN_USER_STMT)
            .bind(user1_uid)
            .bind(user2_uid)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_message_read(&self, to_user: i32, from_user: i32) -> Result<(), String> {
        sqlx::query(UPDATE_MESSAGE_READ_STMT)
            .bind(from_user)
            .bind(to_user)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    pub async fn get_recent_messages(
        &self,
        user_id: i32,
    ) -> Result<Vec<ConversationRecentMessages>, String> {
        sqlx::query_as::<_,ConversationRecentMessages>(GET_MESSAGE_BETWEEN_USER_STMT)
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
}

