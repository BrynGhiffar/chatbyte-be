use crate::{
    app::AppState,
    message::session::WsChatSession,
    middleware::{get_uid_from_header, VerifyToken},
    utility::{ApiResult, ApiSuccess::*},
};
use actix_web::{
    web::{self, get, ServiceConfig},
    Error, HttpRequest, Responder,
};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

pub fn message_config(cfg: &mut ServiceConfig) {
    cfg.route("", get().to(get_messages).wrap(VerifyToken));
    cfg.route(
        "/read",
        web::put().to(update_read_messages).wrap(VerifyToken),
    );
    cfg.route("/ws", get().to(chat_websocket));
}

pub async fn get_messages(
    query: web::Query<MessageReceiver>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> ApiResult<Vec<ClientMessage>> {
    let receiver_uid = query.receiver_uid;
    let uid = get_uid_from_header(req).unwrap();
    let messages = state
        .message_repository
        .get_message_between_users(uid, receiver_uid)
        .await?;
    let messages = messages
        .into_iter()
        .map(|m| ClientMessage {
            id: m.id,
            receiver_id: m.receiver_id,
            sender_id: m.sender_id,
            is_user: (uid == m.sender_id),
            content: m.content,
            time: m.sent_at.format("%H:%M").to_string(),
        })
        .collect::<Vec<_>>();
    Ok(Success(messages))
}

pub async fn update_read_messages(
    query: web::Query<MessageReceiver>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> ApiResult<&'static str> {
    let receiver_uid = query.receiver_uid;
    let uid = get_uid_from_header(req).unwrap();
    state
        .message_repository
        .update_message_read(uid, receiver_uid)
        .await?;
    return Ok(Success("Unread messages"));
}

async fn chat_websocket(
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<ChatWebsocketQuery>,
) -> Result<impl Responder, Error> {
    let ChatWebsocketQuery { token } = query.into_inner();
    log::info!("token: {}", token);
    ws::start(WsChatSession::new(token), &req, stream)
}

// --- UTILITY SRUCTS ---
#[derive(Deserialize)]
struct ChatWebsocketQuery {
    token: String,
}

#[derive(Deserialize)]
pub struct MessageReceiver {
    #[serde(rename = "receiverUid")]
    pub receiver_uid: i32,
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
    pub time: String,
}
