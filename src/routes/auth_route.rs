use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::utility::{ApiError, ApiSuccess};
use crate::{
    app::AppState,
    middleware::{get_uid_from_header, VerifyToken},
    req_model::auth_req_model::LoginForm,
    utility::ApiResult,
};
use actix_web::{
    web::{self, Form, Json, ServiceConfig},
    Either, HttpRequest,
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use ApiError::*;
use ApiSuccess::*;

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
    cfg.route("/valid-token", web::get().to(valid_token).wrap(VerifyToken));
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
        .await?
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

pub fn incorrect_password() -> ApiError {
    BadRequest("Incorrect password".to_string())
}
