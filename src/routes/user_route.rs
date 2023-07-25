use std::collections::HashMap;

use actix_web::{web::{ServiceConfig, self, Json}, Responder, HttpResponse, HttpRequest};
use sea_orm::{ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter, ConnectionTrait, Statement, DatabaseBackend, DatabaseConnection, ActiveValue };
use serde_json::json;
use crate::{app::AppState, middleware::{VerifyToken, get_uid_from_header}, entities::{user_avatar, user}};
use futures::StreamExt;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}

pub async fn update_username(db: &DatabaseConnection, uid: i32, username: String) -> bool {
    let Ok(user) = user::Entity::find_by_id(uid).one(db).await else { return false; };
    let Some(user) = user else { return false; };
    let mut user: user::ActiveModel = user.into();
    user.username = ActiveValue::Set(username);
    let Ok(_) = user.update(db).await else { return false; };
    return true;
}

pub async fn update_email(db: &DatabaseConnection, uid: i32, email: String) -> bool {
    let Ok(user) = user::Entity::find_by_id(uid).one(db).await else { return false; };
    let Some(user) = user else { return false; };
    let mut user: user::ActiveModel = user.into();
    user.email = ActiveValue::Set(email);
    let Ok(_) = user.update(db).await else { return false; };
    return true;
}

pub async fn update_password(db: &DatabaseConnection, uid: i32, password: String) -> bool {
    let Some(exec_res) = db
        .execute(Statement::from_sql_and_values(
            DatabaseBackend::Postgres, r#"
                UPDATE public.user SET password = crypt($1, gen_salt('bf', 5)) where id = $2
            "#, [password.into(), uid.into()]))
        .await.ok() else { return false; };
    if exec_res.rows_affected() == 0 { return false; };
    return true;
}

pub async fn update_user(
    state: web::Data<AppState>,
    body: Json<UpdateUserRequest>,
    req: HttpRequest
) -> impl Responder {
    let db = &state.db;
    let Some(uid) = get_uid_from_header(req) else { return HttpResponse::BadRequest().body("UID is missing from headers")};
    let body = body.into_inner();
    let mut fail_map = HashMap::<String, bool>::new();
    if let Some(username) = body.username {
        let updated = update_username(db, uid, username).await;
        fail_map.insert("username".to_string(), !updated);
    }

    if let Some(email) = body.email { 
        let updated = update_email(db, uid, email).await;
        fail_map.insert("email".to_string(), !updated);
    }

    if let Some(password) = body.password { 
        let updated = update_password(db, uid, password).await;
        fail_map.insert("password".to_string(), !updated);
    }

    if fail_map.values().any(|e| *e) {
        fail_map.retain(|_, v| *v);
        let res = fail_map.keys().map(|k| k.to_owned()).collect::<Vec<String>>().join(", ");
        return HttpResponse::InternalServerError().body(format!("Failed to update {}", res));
    }

    return HttpResponse::Ok().body("Successfully updated user");
}

pub async fn upload_avatar(
    state: web::Data<AppState>,
    req: HttpRequest,
    mut body: web::Payload
) -> impl Responder {
    let db = &state.db;
    let Some(uid) = get_uid_from_header(req) else {
        return HttpResponse::BadRequest().json(json!({
            "success": true,
            "message": "Success uploading image"
        }));
    };
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
    }
    let bytes = bytes.to_vec();
    let new_avatar = user_avatar::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(uid),
        avatar_image: sea_orm::ActiveValue::Set(bytes),
        ..Default::default()
    };
    let Some(_) = new_avatar.insert(db).await.ok() else {
        return HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Error uploading image"
        }));
    };
    return HttpResponse::Created().json(json!({
        "success": true,
        "message": "Image successfully uploaded"
    }));
}

pub async fn get_avatar(
    uid: web::Path<i32>,
    state: web::Data<AppState>
) -> impl Responder {
    let db = &state.db;
    let empty_profile = state.empty_profile.clone();
    let uid = uid.into_inner();
    let Ok(avatar) = user_avatar::Entity::find().filter(user_avatar::Column::UserId.eq(uid)).one(db).await else {
        return HttpResponse::InternalServerError().body(empty_profile);
    };
    let Some(avatar) = avatar else {
        return HttpResponse::InternalServerError().body(empty_profile);
    };
    return HttpResponse::Ok().body(avatar.avatar_image);
}

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::put().to(update_user).wrap(VerifyToken));
    cfg.route("/avatar", web::post().to(upload_avatar).wrap(VerifyToken));
    cfg.route("/avatar/{uid}", web::get().to(get_avatar));
}