use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Json;
use axum::Router;

use crate::app::AppState;
use crate::routes::AuthorizedUser;
use crate::routes::ServerResponse;
use crate::routes::ServerResponse::*;
use crate::service::AuthenticationToken;
use crate::service::ChangePasswordForm;
use crate::service::ChangePasswordSuccess;
use crate::service::LoginForm;
use crate::service::RegisterForm;
use crate::service::RegisterSuccess;

use super::ValidTokenSuccess;

pub fn auth_route(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/change-password", put(change_password))
        .route("/valid-token", get(valid_token))
        .with_state(state)
}

async fn login(
    State(state): State<AppState>,
    body: Result<Json<LoginForm>, JsonRejection>,
) -> ServerResponse<String> {
    let Json(login_form) = match body {
        Ok(payload) => payload,
        Err(e) => return Failed(anyhow!(e)),
    };
    let res = state.auth_service.login(login_form).await;
    match res {
        Ok(AuthenticationToken(token)) => Success(token),
        Err(e) => Failed(e),
    }
}

async fn register(
    State(state): State<AppState>,
    body: Result<Json<RegisterForm>, JsonRejection>,
) -> ServerResponse<RegisterSuccess> {
    let Json(register_form) = match body {
        Ok(payload) => payload,
        Err(e) => return Failed(anyhow!(e.to_string())),
    };
    let res = state.auth_service.register(register_form).await;
    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e),
    }
}

async fn change_password(
    State(state): State<AppState>,
    AuthorizedUser { user_id }: AuthorizedUser,
    body: Result<Json<ChangePasswordForm>, JsonRejection>,
) -> ServerResponse<ChangePasswordSuccess> {
    let Json(change_password_form) = match body {
        Ok(payload) => payload,
        Err(e) => return Failed(anyhow!(e.to_string())),
    };
    let res = state
        .auth_service
        .change_password(user_id, change_password_form)
        .await;
    match res {
        Ok(r) => Success(r),
        Err(e) => Failed(e),
    }
}

async fn valid_token(_: AuthorizedUser) -> ServerResponse<ValidTokenSuccess> {
    Success(ValidTokenSuccess)
}
