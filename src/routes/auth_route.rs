use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH, Duration}};

use actix_web::{ Either, web::{Json, Form, ServiceConfig, self}, HttpRequest};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait };
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use jwt::SignWithKey;
use crate::{req_model::auth_req_model::LoginForm, entities::user, app::AppState, utility:: ApiResult, middleware::{get_uid_from_header, VerifyToken}};
use crate::utility::{ApiError, ApiSuccess};

use ApiError::*;
use ApiSuccess::*;

pub fn email_not_found(email: String) -> ApiError {
    BadRequest(format!("User with email {:?} is not found", email.clone()))
}

pub fn incorrect_password() -> ApiError {
    BadRequest("Incorrect password".to_string())
}

async fn login(
    state: web::Data<AppState>,
    body: Either<Json<LoginForm>, Form<LoginForm>>
) -> ApiResult<String> {

    let LoginForm { email, password } = body.into_inner();
    let state = state.into_inner();
    let tuser = user::Entity::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(&state.db).await?
        .ok_or(email_not_found(email.clone()))?;
    let valid = bcrypt::verify(&password, &tuser.password).map_err(|e| ServerError(e.to_string()))?;
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

async fn valid_token(
    req: HttpRequest 
) -> ApiResult<&'static str> {
   let _ = get_uid_from_header(req).expect("user id is missing from header");
   return Ok(Success("Token is valid"));
}

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
    cfg.route("/valid-token", web::get().to(valid_token).wrap(VerifyToken));
}