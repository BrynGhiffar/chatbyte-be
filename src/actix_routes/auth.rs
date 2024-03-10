use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    app::AppState,
    middleware::{get_uid_from_header, VerifyToken},
    req_model::auth_req_model::LoginForm,
    utility::ApiResult,
};
use crate::{
    req_model::auth_req_model::{ChangePasswordForm, RegisterForm},
    utility::{ApiError, ApiSuccess},
};
use actix_web::{
    web::{self, Form, Json, ServiceConfig},
    Either, HttpRequest,
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use regex::Regex;
use sha2::Sha256;

use ApiError::*;
use ApiSuccess::*;

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
    cfg.route("/register", web::post().to(register));
    cfg.route("/valid-token", web::get().to(valid_token).wrap(VerifyToken));
    cfg.route(
        "/change-password",
        web::put().to(change_password).wrap(VerifyToken),
    );
}

async fn login(
    state: web::Data<AppState>,
    body: Either<Json<LoginForm>, Form<LoginForm>>,
) -> ApiResult<String> {
    let LoginForm { email, password } = body.into_inner();
    let state = state.into_inner();
    let tuser = state
        .auth_repository
        .find_user_by_email(email.clone())
        .await
        .map_err(ServerError)?
        .ok_or(email_not_found(email.clone()))?;
    let valid =
        bcrypt::verify(&password, &tuser.password).map_err(|e| ServerError(e.to_string()))?;
    if !valid {
        return Err(incorrect_password());
    }
    let secret = state.env_jwt_secret.clone();
    let expiration = state.env_jwt_secret_mins.clone();
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp_secs = (since_the_epoch + Duration::from_secs(expiration * 60)).as_secs();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("uid".to_string(), u64::try_from(tuser.id).ok().unwrap());
    claims.insert("expiration".to_string(), timestamp_secs);
    let payload = claims.sign_with_key(&key).unwrap();
    return Ok(Success(payload));
}

async fn valid_token(req: HttpRequest) -> ApiResult<&'static str> {
    let _ = get_uid_from_header(req).expect("user id is missing from header");
    return Ok(Success("Token is valid"));
}

pub fn email_not_found(email: String) -> ApiError {
    BadRequest(format!("User with email {:?} is not found", email.clone()))
}

pub fn email_already_registered(email: String) -> ApiError {
    BadRequest(format!(
        "User with email {:?} is already registered",
        email.clone()
    ))
}

pub fn invalid_email(email: String) -> ApiError {
    BadRequest(format!("Email '{:?}' is invalid", email.clone()))
}

pub fn incorrect_password() -> ApiError {
    BadRequest("Incorrect password".to_string())
}

pub fn password_too_short() -> ApiError {
    BadRequest("Password length must be at least 5 characters ".to_string())
}

pub fn new_password_too_short() -> ApiError {
    BadRequest("New password length must be at least 5 characters long".to_string())
}

pub async fn register(
    state: web::Data<AppState>,
    body: Either<Json<RegisterForm>, Form<RegisterForm>>,
) -> ApiResult<String> {
    let RegisterForm { email, password } = body.into_inner();
    let regex = Regex::new(r#"(?m)^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#).unwrap();
    if !regex.is_match(&email) {
        return Err(invalid_email(email));
    }
    let res = state
        .auth_repository
        .find_user_by_email(email.clone())
        .await;
    let user = res.map_err(|e| e.to_string()).map_err(BadRequest)?;
    if let Some(_) = user {
        return Err(email_already_registered(email));
    }

    if password.len() < 5 {
        return Err(password_too_short());
    }

    let success = state
        .auth_repository
        .create_user(email, password)
        .await
        .map_err(ServerError)?;
    if !success {
        return Err(ServerError("A database error occurred".to_string()));
    }
    return Ok(Success("Successfully registered".to_string()));
}

pub async fn change_password(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: Either<Json<ChangePasswordForm>, Form<ChangePasswordForm>>,
) -> ApiResult<String> {
    let ChangePasswordForm {
        old_password,
        new_password,
    } = body.into_inner();
    let uid = get_uid_from_header(req).unwrap();
    let user = state
        .auth_repository
        .find_user_by_id(uid)
        .await
        .map_err(|e| e.to_string())
        .map_err(ServerError)?;
    let Some(user) = user else {
        return Err(BadRequest("User not found".to_string()));
    };
    let valid =
        bcrypt::verify(&old_password, &user.password).map_err(|e| ServerError(e.to_string()))?;
    if !valid {
        return Err(incorrect_password());
    }
    if new_password.len() < 5 {
        return Err(new_password_too_short());
    }
    let res = state
        .auth_repository
        .update_password(uid, new_password)
        .await
        .map_err(BadRequest)?;
    if !res {
        return Err(ServerError("Failed to update password".to_string()));
    }
    return Ok(Success("Password updated successfully".to_string()));
}
