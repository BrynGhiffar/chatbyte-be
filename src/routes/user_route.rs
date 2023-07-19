use actix_web::{web::{ServiceConfig, self}, Responder, HttpResponse, HttpRequest};
use crate::{app::AppState, middleware::{VerifyToken, get_uid_from_header}};
use futures::StreamExt;

pub async fn update_user(
    state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    let uid = get_uid_from_header(req);
    return HttpResponse::BadRequest().body("Endpoint is being built");
}

pub async fn upload_avatar(
    state: web::Data<AppState>,
    req: HttpRequest,
    mut body: web::Payload
) -> impl Responder {
    let uid = get_uid_from_header(req);
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
    }
    return HttpResponse::Ok().body(bytes);
}

pub async fn get_avatar(
    uid: web::Path<i32>,
    state: web::Data<AppState>
) -> impl Responder {

    return HttpResponse::BadRequest().body("TODO");
}

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::put().to(update_user).wrap(VerifyToken));
    cfg.route("/avatar", web::post().to(upload_avatar).wrap(VerifyToken));
    cfg.route("/avatar/{uid}", web::get().to(get_avatar));
}