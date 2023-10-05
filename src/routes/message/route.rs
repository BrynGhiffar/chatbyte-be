use axum::Router;
use axum::extract::State;
use axum::routing::get;

use crate::app::AppState;
use crate::routes::AuthorizedUser;
use crate::routes::ServerResponse;
use crate::routes::ServerResponse::*;
use crate::service::DirectMessageModel;
use crate::service::GroupMessageModel;

use super::GroupIdQuery;
use super::ReceiverUidQuery;

pub fn message_route(state: AppState) -> Router {
    Router::new()
        .route("/", get(find_direct_message))
        .route("/group", get(find_group_message))
        .with_state(state)
}

pub async fn find_direct_message(
    AuthorizedUser { user_id }: AuthorizedUser,
    ReceiverUidQuery { receiver_uid }: ReceiverUidQuery,
    State(state): State<AppState>
) -> ServerResponse<Vec<DirectMessageModel>> 
{
    let res = state.message_service
        .find_direct_message(
            user_id, receiver_uid
        )
        .await;
    match res {
        Ok(res) => Success(res),
        Err(e) => Failed(e)
    }
}

pub async fn find_group_message(
    AuthorizedUser { user_id }: AuthorizedUser,
    GroupIdQuery { group_id }: GroupIdQuery,
    State(state): State<AppState>
) -> ServerResponse<Vec<GroupMessageModel>> {
    let res = state
        .message_service
        .find_group_message(user_id, group_id)
        .await;
    match res {
        Ok(res) => Success(res),
        Err(e) => Failed(e)
    }
}