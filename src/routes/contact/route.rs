use axum::extract::State;
use axum::routing::get;
use axum::Router;

use crate::app::AppState;
use crate::routes::AuthorizedUser;
use crate::routes::ServerResponse;
use crate::routes::ServerResponse::*;
use crate::service::DirectContact;
use crate::service::DirectConversation;
use crate::service::GroupContact;
use crate::service::GroupConversation;

pub fn contact_route(state: AppState) -> Router {
    Router::new()
        .route("/direct", get(find_direct_contact_for_user))
        .route("/direct/recent", get(find_direct_conversation_for_user))
        .route("/group", get(find_group_contact_for_user))
        .route("/group/recent", get(find_group_conversation_for_user))
        .with_state(state)
}

async fn find_direct_contact_for_user(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
) -> ServerResponse<Vec<DirectContact>> {
    let res = state
        .contact_service
        .find_direct_contacts_for_user(user_id)
        .await;
    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e),
    }
}

async fn find_group_contact_for_user(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
) -> ServerResponse<Vec<GroupContact>> {
    let res = state
        .contact_service
        .find_group_contacts_for_user(user_id)
        .await;
    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e),
    }
}

async fn find_direct_conversation_for_user(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
) -> ServerResponse<Vec<DirectConversation>> {
    let res = state
        .contact_service
        .find_direct_conversations_for_user(user_id)
        .await;

    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e),
    }
}

async fn find_group_conversation_for_user(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
) -> ServerResponse<Vec<GroupConversation>> {
    let res = state
        .contact_service
        .find_group_conversations_for_user(user_id)
        .await;

    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e),
    }
}
