use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH, Duration}};

use actix_web::{Responder, Either, web::{Json, Form, ServiceConfig, self}, HttpResponse};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait };
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use jwt::SignWithKey;
use crate::{req_model::auth_req_model::LoginForm, res_model::auth_res_model::LoginResponse, entities::user, app::AppState};

fn fail_ise(msg: &str) -> HttpResponse {
    return HttpResponse::InternalServerError()
        .json(LoginResponse::failed(msg.to_string()));
}

fn fail_br(msg: &str) -> HttpResponse {
    return HttpResponse::BadRequest()
        .json(LoginResponse::failed(msg.to_string()));
}

async fn login(
    state: web::Data<AppState>,
    body: Either<Json<LoginForm>, Form<LoginForm>>
) -> impl Responder {
    let LoginForm { email, password } = body.into_inner();
    let state = state.into_inner();
    let Ok(res) = user::Entity::find().filter(user::Column::Email.eq(email.clone())).one(&state.db).await else {
        return fail_ise("Error when fetching users");
    };
    let Some(tuser) = res else {
        return fail_br(format!("User with email {:?} is not found", email.clone()).as_str()); 
    };
    let Ok(valid) = bcrypt::verify(&password, &tuser.password) else {
        return fail_ise("Failed verifying password");
    };
    if !valid {
        return fail_br("Incorrect password");
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
    return HttpResponse::Ok().json(LoginResponse::success(payload));
}

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
}