use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub fn bad_request(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": message.to_string()
    }))
}

pub fn server_error(message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "success": false,
        "message": message.to_string()
    }))
}

pub fn success<T: Serialize>(payload: T) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "payload": payload
    }))
}
