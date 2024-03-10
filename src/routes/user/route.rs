use axum::Json;
use axum::Router;
use axum::extract::rejection::JsonRejection;
use axum::routing::get;
use axum::routing::put;
use axum::routing::post;
use axum::extract::Path;
use axum::extract::State;
use axum::body::Bytes;

use crate::app::AppState;
use crate::routes::AuthorizedUser;
use crate::routes::ImageResponse;
use crate::routes::ServerResponse;
use crate::routes::ServerResponse::*;
use crate::service::SuccessfullyUpdateUser;
use crate::service::UserDetail;

use super::ChangeUsernameForm;

pub fn user_route(state: AppState) -> Router {
    Router::new()
        .route("/details", get(find_user_details))
        .route("/details", put(update_username))
        .route("/avatar/:user_id", get(find_user_profile))
        .route("/avatar", post(update_avatar))
        .with_state(state)
}

pub async fn update_username(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
    body: Result<Json<ChangeUsernameForm>, JsonRejection>,
) -> ServerResponse<SuccessfullyUpdateUser> {
    let Json(ChangeUsernameForm { username }) = match body {
        Ok(form) => form,
        Err(e) => return Failed(e.into())
    };
    let res = state.user_service
        .update_username(user_id, username)
        .await;
    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e)
    }
}

pub async fn find_user_details(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>
) -> ServerResponse<UserDetail> {
    let res = state.user_service
        .find_user_details(user_id)
        .await;

    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e)
    }
}

pub async fn find_user_profile(
    Path(user_id): Path<i32>,
    State(state): State<AppState>
) -> ImageResponse {
    let image = state
        .user_service
        .find_user_avatar(user_id)
        .await
        .unwrap_or(state.empty_profile)
        ;
    ImageResponse(image)
}

pub async fn update_avatar(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
    body: Bytes,
) -> ServerResponse<SuccessfullyUpdateUser> {
    let profile_picture = body.into_iter().collect::<Vec<_>>();
    let res = state.user_service
        .update_user_avatar(user_id, profile_picture)
        .await;
    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e)
    }
}