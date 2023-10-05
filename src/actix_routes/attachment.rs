use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest,
};

use crate::{
    app::AppState,
    utility::{body_to_bytes, ApiError::*, ApiResult},
};

pub fn attachment_config(cfg: &mut ServiceConfig) {}

pub async fn upload_attachment(
    state: web::Data<AppState>,
    body: web::Payload,
) -> ApiResult<String> {
    let content = body_to_bytes(body).await;
    Err(BadRequest("Endpoint in works".to_string()))
}

pub async fn link_attachment() -> ApiResult<String> {
    Err(BadRequest("Endpoint in works".to_string()))
}
