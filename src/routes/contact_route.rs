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
    pub content: String,
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
                select
                    "Message"."id" as id,
                    "Message"."sentAt" as sent_at,
                    "Message"."receiverId" as receiver_id,
                    "Message"."senderId" as sender_id,
                    "Message"."content" as content,
                    lasts."unreadCount" as unread_count,
                    "User"."username" as username
                from "Message" 
                join "User" on
                    (case when "Message"."senderId" = $1 then "Message"."receiverId" else "Message"."senderId" end) = "User"."id"
                join (
                    select 
                        max(tbl."id") as "messageId",
                        sum(case when (tbl."read" = false and tbl."senderId" != $1) then 1 else 0 end) as "unreadCount"
                    from (
                        select
                            "Message"."id",
                            "Message"."read",
                            "Message"."senderId",
                            case when "Message"."senderId" < "Message"."receiverId" then "Message"."senderId" else "Message"."receiverId" end as c1,
                            case when "Message"."senderId" < "Message"."receiverId" then "Message"."receiverId" else "Message"."senderId" end as c2
                        from "Message"
                    ) as tbl
                    group by c1, c2
                ) as lasts on "Message"."id" = lasts."messageId"
                where "Message"."senderId" = $1 or "Message"."receiverId" = $1;
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
            "content": m.content,
            "unread_count": m.unread_count
        })
    }).collect::<Vec<_>>();

    return success(msg);
}


pub fn contact_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_contacts).wrap(VerifyToken));
    cfg.route("/recent", web::get().to(recent_conversation).wrap(VerifyToken));
}
