use std::fmt::Display;
use std::fmt::Debug;

use actix_web::Responder;
use actix_web::{HttpResponse, ResponseError, body::BoxBody};
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;

pub fn bad_request<T: ToString>(message: T) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "success": false,
        "message": message.to_string()
    }))
}

pub fn server_error<T: ToString>(message: T) -> HttpResponse {
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



#[derive(Debug)]
pub enum ApiError {
    ServerError(String),
    BadRequest(String)
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ApiError::*;
        match self {
            BadRequest(e) => writeln!(f, "{}", e),
            ServerError(e) => writeln!(f, "{}", e)
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        use ApiError::*;
        ServerError(value.to_string())
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::BadRequest(message) => bad_request(message),
            Self::ServerError(message) => server_error(message)
        }
    }
}

pub enum ApiSuccess<T: Serialize> {
    Success(T)
}

impl<T: Serialize> Responder for ApiSuccess<T> {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Self::Success(payload) => success(payload)
        }
    }
}

pub type ApiResult<T> = Result<ApiSuccess<T>, ApiError>;