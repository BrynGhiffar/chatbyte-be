use std::collections::BTreeMap;

use actix_web::{Responder, Either, web::{Json, Form, ServiceConfig, self}, HttpResponse};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait };
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use jwt::SignWithKey;
use crate::{req_model::auth_req_model::LoginForm, res_model::auth_res_model::LoginResponse, entities::user, app::AppState};

async fn login(
    state: web::Data<AppState>,
    body: Either<Json<LoginForm>, Form<LoginForm>>
) -> impl Responder {
    let LoginForm { email, password } = body.into_inner();
    let state = state.into_inner();
    let Ok(res) = user::Entity::find().filter(user::Column::Email.eq(email.clone())).one(&state.db).await else {
        return HttpResponse::InternalServerError().json(
            LoginResponse::failed("Error when fetching users".to_string())
        );
    };
    let Some(tuser) = res else { 
        return HttpResponse::BadRequest().json(
            LoginResponse::failed(
                format!("User with email {:?} is not found", email.clone())
            )
        );
    };
    let Ok(valid) = bcrypt::verify(&password, &tuser.password) else {
        return HttpResponse::InternalServerError().json(
            LoginResponse::failed(
                "Internal Server Error".to_string()
            )
        );
    };
    if !valid {
        return HttpResponse::BadRequest().json(
            LoginResponse::failed(
                "Incorrect password".to_string()
            )
        );
    }
    let Ok(secret) = std::env::var("JWT_SECRET") else {
        return HttpResponse::InternalServerError().json(
            LoginResponse::failed("JWT_SECRET is missing".to_string())
        )
    };
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("uid".to_string(), tuser.id);
    let payload = claims.sign_with_key(&key).unwrap();
    return HttpResponse::Ok().json(LoginResponse::success(payload));
}

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
}