use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest,
};
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect, QueryFilter, ColumnTrait, Statement, DbBackend};
use serde::Serialize;

use crate::{
    app::AppState,
    entities::user,
    utility::{ApiResult, ApiSuccess}, middleware::{VerifyToken, get_uid_from_header},
};

#[derive(FromQueryResult, Serialize)]
struct Contact {
    id: i32,
    email: String,
    username: String,
}

async fn get_contacts(req: HttpRequest, state: web::Data<AppState>) -> ApiResult<Vec<Contact>> {
    use ApiSuccess::*;

    let db = &state.db;
    let uid = get_uid_from_header(req).unwrap();
    let contacts = user::Entity::find()
        .filter(user::Column::Id.ne(uid))
        .select_only()
        .column(user::Column::Id)
        .column(user::Column::Email)
        .column(user::Column::Username)
        .into_model::<Contact>()
        .all(db)
        .await?;

    Ok(Success(contacts))
}

#[derive(FromQueryResult)]
pub struct ConversationRecentMessages {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    // pub sender_id: i32,
    // pub receiver_id: i32,
    pub contact_id: i32,
    pub last_message: String,
    pub unread_count: i64,
    pub username: String
}

#[derive(Serialize)]
pub struct RecentConversation {
    username: String,
    sent_at: String,
    contact_id: i32,
    content: String,
    unread_count: i64
}

impl From<&ConversationRecentMessages> for RecentConversation {
    fn from(value: &ConversationRecentMessages) -> Self {
        RecentConversation { 
            username: value.username.clone(), 
            sent_at: value.sent_at.format("%H:%M").to_string(), 
            contact_id: value.contact_id, 
            content: value.last_message.clone(), 
            unread_count: value.unread_count 
        } 
    }
}

pub async fn recent_conversation(
    state: web::Data<AppState>,
    req: HttpRequest
) -> ApiResult<Vec<RecentConversation>> {
    use ApiSuccess::*;
    let state = state.into_inner();
    let db = &state.db;
    let uid = get_uid_from_header(req).unwrap();
    let res = ConversationRecentMessages::find_by_statement(
        Statement::from_sql_and_values(
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
            [uid.into()]
        ))
        .all(db)
        .await?;
    let recent_conversations = res.iter()
        .map(|r| RecentConversation::from(r))
        .collect::<Vec<_>>();

    Ok(Success(recent_conversations))
}


pub fn contact_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_contacts).wrap(VerifyToken));
    cfg.route("/recent", web::get().to(recent_conversation).wrap(VerifyToken));
}
