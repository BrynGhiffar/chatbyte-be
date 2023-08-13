use chrono::NaiveDateTime;
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait,
    FromQueryResult, QueryFilter, QueryOrder, Statement,
};

use crate::entities::message;

#[derive(Clone)]
pub struct MessageRepository {
    conn: DatabaseConnection,
}

impl MessageRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        MessageRepository { conn }
    }

    pub async fn get_message_between_users(
        &self,
        user1_uid: i32,
        user2_uid: i32,
    ) -> Result<Vec<message::Model>, DbErr> {
        let messages = message::Entity::find()
            .filter(
                message::Column::ReceiverId
                    .eq(user1_uid)
                    .and(message::Column::SenderId.eq(user2_uid))
                    .or(message::Column::ReceiverId
                        .eq(user2_uid)
                        .and(message::Column::SenderId.eq(user1_uid))),
            )
            .order_by_asc(message::Column::SentAt)
            .all(&self.conn)
            .await?;
        Ok(messages)
    }

    pub async fn update_message_read(&self, to_user: i32, from_user: i32) -> Result<(), DbErr> {
        message::Entity::update_many()
            .col_expr(message::Column::Read, Expr::value(true))
            .filter(
                message::Column::ReceiverId
                    .eq(to_user)
                    .and(message::Column::SenderId.eq(from_user)),
            )
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    pub async fn get_recent_messages(
        &self,
        user_id: i32,
    ) -> Result<Vec<ConversationRecentMessages>, DbErr> {
        let res = ConversationRecentMessages::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                    SELECT
                        unread_message_content.id,
                        CASE WHEN sender_id = $1 THEN receiver_id ELSE sender_id END as contact_id,
                        sent_at, 
                        CASE WHEN receiver_id != $1 THEN 0 ELSE unread_count END,
                        last_message,
                        public.user.username
                    FROM unread_message_content
                    JOIN public.user ON 
                        CASE WHEN least(receiver_id, sender_id) = $1 
                            THEN GREATEST(receiver_id, sender_id) = public.user.id 
                            ELSE LEAST(receiver_id, sender_id) = public.user.id 
                        END
                    WHERE $1 in (sender_id, receiver_id)
                    ;
                    "#,
            [user_id.into()],
        ))
        .all(&self.conn)
        .await?;
        Ok(res)
    }
}

// --- UTILITY STRUCTS ---
#[derive(FromQueryResult)]
pub struct ConversationRecentMessages {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    // pub sender_id: i32,
    // pub receiver_id: i32,
    pub contact_id: i32,
    pub last_message: String,
    pub unread_count: i64,
    pub username: String,
}
