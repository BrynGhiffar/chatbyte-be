use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Clone)]
pub struct MessageRepositoryModel {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub read: bool,
    pub edited: bool,
    pub deleted: bool,
}

#[derive(sqlx::FromRow)]
pub struct ConversationRecentMessageRepositoryModel {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    // pub sender_id: i32,
    // pub receiver_id: i32,
    pub contact_id: i32,
    pub last_message: String,
    pub unread_count: i64,
    pub username: String,
    pub deleted: bool,
}
