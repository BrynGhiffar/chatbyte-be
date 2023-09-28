use crate::{
    app::AppState,
    middleware::{get_uid_from_header, VerifyToken},
    utility::{ApiResult, ApiSuccess::*, ApiError::*},
    websocket::message::AppMessage,
    websocket::server::WsRequest,
};
use actix_web::{
    web::{self, get, ServiceConfig},
    HttpRequest,
};
use serde::{Deserialize, Serialize};

pub fn message_config(cfg: &mut ServiceConfig) {
    cfg.route("", get().to(get_messages).wrap(VerifyToken));
    cfg.route(
        "/read",
        web::put().to(update_read_messages).wrap(VerifyToken),
    );
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
        .await
        .map_err(ServerError)?
        ;
    let messages = messages
        .into_iter()
        .map(|m| {
            let content = if m.deleted { String::from("") } else { m.content.clone() };

            ClientMessage {
                id: m.id,
                receiver_id: m.receiver_id,
                sender_id: m.sender_id,
                is_user: (uid == m.sender_id),
                deleted: m.deleted,
                content,
                edited: m.edited,
                time: m.sent_at.format("%H:%M").to_string(),
                receiver_read: m.read,
            }
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
    // since the session id
    let message = WsRequest::ReadMessage {
        receiver_uid: uid,
        sender_uid: receiver_uid,
    };
    let message = AppMessage::Message {
        session_id: uid,
        message: message.to_string(),
    };
    state.session_factory.app_tx.send(message).unwrap();
    state
        .message_repository
        .update_message_read(uid, receiver_uid)
        .await
        .map_err(ServerError)?;
    return Ok(Success("Unread messages"));
}

// --- UTILITY SRUCTS ---
#[derive(Deserialize)]
pub struct MessageReceiver {
    #[serde(rename = "receiverUid")]
    pub receiver_uid: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMessage {
    pub id: i32,
    pub receiver_id: i32,
    pub sender_id: i32,
    pub is_user: bool,
    pub content: String,
    pub time: String,
    pub receiver_read: bool,
    pub edited: bool,
    pub deleted: bool,
}
