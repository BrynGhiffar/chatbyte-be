use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn healthcheck() -> impl IntoResponse {
    Json(json!({
        "success": true,
        "payload": "Chat app backend is live"
    }))
}
