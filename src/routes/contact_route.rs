use actix_web::{
    web::{self, ServiceConfig},
    Responder, HttpRequest,
};
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect, QueryFilter, ColumnTrait, Statement, DbBackend};
use serde::Serialize;
use serde_json::json;

use crate::{
    app::AppState,
    entities::user,
    utility::{server_error, success, bad_request}, middleware::{VerifyToken, get_uid_from_header},
};

#[derive(FromQueryResult, Serialize)]
struct Contact {
    id: i32,
    email: String,
    username: String,
}

async fn get_contacts(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
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
        .await;
    let Some(contacts) = contacts.ok() else {
        return server_error("A database error occurred");
    };

    return success(contacts);
}

#[derive(FromQueryResult)]
pub struct ConversationRecentMessages {
    pub id: i32,
    pub sent_at: NaiveDateTime,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub last_message: String,
    pub unread_count: i64,
    pub username: String
}

pub async fn recent_conversation(
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let state = state.into_inner();
    let db = &state.db;
    let Some(uid) = get_uid_from_header(req) else { return bad_request("token is missing") };
    let res = ConversationRecentMessages::find_by_statement(
        Statement::from_sql_and_values(
            DbBackend::Postgres, 
            r#"
                SELECT
                    unread_message_content.id,
                    sender_id,
                    receiver_id, sent_at, 
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
        .await;

    let Some(msg) = res.ok() else {
        return server_error("Database error");
    };
    let msg = msg.iter().map(|m| {
        let sent_at = m.sent_at.format("%H:%M").to_string();
        let contact_id = if m.sender_id == uid { m.receiver_id } else { m.sender_id };
        json!({
            "username": m.username,
            "sent_at": sent_at,
            "contact_id": contact_id,
            "content": m.last_message,
            "unread_count": m.unread_count
        })
    }).collect::<Vec<_>>();

    return success(msg);
}


pub fn contact_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_contacts).wrap(VerifyToken));
    cfg.route("/recent", web::get().to(recent_conversation).wrap(VerifyToken));
}
