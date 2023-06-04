use actix_web::{Responder, Either, web::{Json, Form, ServiceConfig, self}, HttpResponse};

use crate::{req_model::auth_req_model::LoginForm, res_model::auth_res_model::LoginResponse};

async fn login(
    body: Either<Json<LoginForm>, Form<LoginForm>>
) -> impl Responder {

    let res = LoginResponse::failed("Api is being built".to_string());
    return HttpResponse::Ok().json(res);
}

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
}