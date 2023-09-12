use std::collections::HashMap;

use crate::{
    app::AppState,
    middleware::{get_uid_from_header, VerifyToken},
    repository::entities::user,
    utility::{ApiError::*, ApiResult, ApiSuccess::*},
};
use actix_web::{
    web::{self, Json, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use futures::StreamExt;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub async fn update_user(
    state: web::Data<AppState>,
    body: Json<UpdateUserRequest>,
    req: HttpRequest,
) -> ApiResult<&'static str> {
    let uid = get_uid_from_header(req).unwrap();
    let body = body.into_inner();
    let mut fail_map = HashMap::<String, bool>::new();
    if let Some(username) = body.username {
        let updated = state.auth_repository.update_username(uid, username).await;
        fail_map.insert("username".to_string(), !updated);
    }

    if let Some(email) = body.email {
        let updated = state.auth_repository.update_email(uid, email).await;
        fail_map.insert("email".to_string(), !updated);
    }

    if let Some(password) = body.password {
        let updated = state.auth_repository.update_password(uid, password).await;
        fail_map.insert("password".to_string(), !updated);
    }

    if fail_map.values().any(|e| *e) {
        fail_map.retain(|_, v| *v);
        let res = fail_map
            .keys()
            .map(|k| k.to_owned())
            .collect::<Vec<String>>()
            .join(", ");
        return Err(ServerError(format!("Failed to update {}", res)));
    }

    return Ok(Success("Successfully updated user"));
}

pub async fn upload_avatar(
    state: web::Data<AppState>,
    req: HttpRequest,
    mut body: web::Payload,
) -> ApiResult<&'static str> {
    let uid = get_uid_from_header(req).unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
    }
    let bytes = bytes.to_vec();
    state
        .user_repository
        .upsert_user_profile(uid, bytes)
        .await?;

    Ok(Success("Successfully updated image"))
}

pub async fn get_avatar(uid: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let empty_profile = state.empty_profile.clone();
    let uid = uid.into_inner();
    let Ok(Some(avatar)) = state.user_repository.get_avatar(uid).await else {
        return HttpResponse::Ok().body(empty_profile);
    };
    return HttpResponse::Ok().body(avatar.avatar_image);
}

pub async fn get_user_details(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> ApiResult<UserDetail> {
    let db = &state.db;
    let uid = get_uid_from_header(req).unwrap();

    let Some(model) = user::Entity::find_by_id(uid).one(db).await? else {
        return Err(BadRequest("user not found".to_string()));
    };

    Ok(Success(UserDetail {
        uid: model.id,
        username: model.username,
    }))
}

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::put().to(update_user).wrap(VerifyToken));
    cfg.route(
        "/details",
        web::get().to(get_user_details).wrap(VerifyToken),
    );
    cfg.route("/avatar", web::post().to(upload_avatar).wrap(VerifyToken));
    cfg.route("/avatar/{uid}", web::get().to(get_avatar));
}

// --- UTILITY STRUCTS ---
#[derive(Serialize)]
pub struct UserDetail {
    uid: i32,
    username: String,
}
