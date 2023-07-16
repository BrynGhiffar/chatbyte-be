use actix_web::{HttpRequest, Responder, web::{ServiceConfig, get, self}, Error};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder, sea_query::Expr};

use crate::{middleware::{VerifyToken, get_uid_from_header}, app::AppState, entities::message, utility::{bad_request, server_error, success}, message::session::WsChatSession};

#[derive(Deserialize)]
pub struct MessageReceiver {
    #[serde(rename = "receiverUid")]
    pub receiver_uid: String
}

#[derive(Serialize)]
pub struct ClientMessage {
    pub id: i32,
    #[serde(rename = "receiverId")]
    pub receiver_id: i32,
    #[serde(rename = "senderId")]
    pub sender_id: i32,
    #[serde(rename = "isUser")]
    pub is_user: bool,
    pub content: String,
    pub time: String
}

pub async fn get_messages(
    query: web::Query<MessageReceiver>,
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let db = &state.db;
    let receiver_uid = query.receiver_uid.clone();
    let Some(receiver_uid) = receiver_uid.parse::<i32>().ok() else {
        return bad_request("invalid receiver uid");
    };
    let uid = req.headers().get("uid")
        .map(|v| v.to_str().ok()).flatten()
        .map(|s| s.to_string())
        .map(|s| s.parse::<i32>().ok()).flatten().unwrap();
    log::info!("uid: {}", uid);
    let messages = message::Entity::find()
        .filter(
            message::Column::ReceiverId.eq(uid)
                .and(message::Column::SenderId.eq(receiver_uid))
                .or(
                    message::Column::ReceiverId.eq(receiver_uid)
                    .and(message::Column::SenderId.eq(uid))
                )
        )
        .order_by_asc(message::Column::SentAt)
        .all(db)
        .await
        ;
    let Ok(messages) = messages else {
        return server_error("Database error occurred");
    };
    let messages = messages.into_iter().map(|m| ClientMessage {
        id: m.id,
        receiver_id: m.receiver_id,
        sender_id: m.sender_id,
        is_user: (uid == m.sender_id),
        content: m.content,
        time: m.sent_at.format("%H:%M").to_string()
    }).collect::<Vec<_>>();
    return success(messages);
}

pub async fn update_read_messages(
    query: web::Query<MessageReceiver>,
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder{
    let Some(receiver_uid) = query.into_inner().receiver_uid.parse::<i32>().ok() else {
        return bad_request("invalid receiverUid");
    };
    let db = &state.db;
    let uid = get_uid_from_header(req).unwrap();
    let res = message::Entity::update_many()
        .col_expr(message::Column::Read, Expr::value(true))
        .filter(message::Column::ReceiverId.eq(uid).and(message::Column::SenderId.eq(receiver_uid)))
        .exec(db)
        .await;
    let Some(_) = res.ok() else {
        return server_error("Database error occurred");
    };
    return success("Unread messages");
}

#[derive(Deserialize)]
struct ChatWebsocketQuery {
    token: String
}
async fn chat_websocket(
    req: HttpRequest, 
    stream: web::Payload,
    query: web::Query<ChatWebsocketQuery>
) -> Result<impl Responder, Error> {
    let ChatWebsocketQuery { token } = query.into_inner();
    log::info!("token: {}", token);
    ws::start(WsChatSession::new(token), &req, stream)
}

pub fn message_config(cfg: &mut ServiceConfig) {
    cfg.route("", get().to(get_messages).wrap(VerifyToken));
    cfg.route("/read", web::put().to(update_read_messages).wrap(VerifyToken));
    cfg.route("/ws", get().to(chat_websocket));
    // cfg.service(web::)
}