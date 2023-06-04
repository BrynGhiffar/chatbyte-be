use actix_web::{ Responder, get };

#[get("")]
async fn healthcheck() -> impl Responder {
    "API is healthy"
}