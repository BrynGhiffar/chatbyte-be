use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest,
};
use serde::Serialize;

use crate::{
    app::AppState,
    utility::{ApiResult, ApiSuccess}, middleware::{VerifyToken, get_uid_from_header}, repository::{message_repository::ConversationRecentMessages, contact_repository::Contact},
};

pub fn contact_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_contacts).wrap(VerifyToken));
    cfg.route("/recent", web::get().to(recent_conversation).wrap(VerifyToken));
}

async fn get_contacts(
    req: HttpRequest, 
    state: web::Data<AppState>
) -> ApiResult<Vec<Contact>> {
    use ApiSuccess::*;
    let uid = get_uid_from_header(req).unwrap();
    let contacts = state.contact_repository.get_contacts(uid).await?;

    Ok(Success(contacts))
}

pub async fn recent_conversation(
    state: web::Data<AppState>,
    req: HttpRequest
) -> ApiResult<Vec<RecentConversation>> {
    use ApiSuccess::*;
    let state = state.into_inner();
    let uid = get_uid_from_header(req).unwrap();
    let res = state.message_repository.get_recent_messages(uid).await?;
    let recent_conversations = res.iter()
        .map(RecentConversation::from)
        .collect::<Vec<_>>();

    Ok(Success(recent_conversations))
}

// --- UTILITY STRUCTS ---
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

