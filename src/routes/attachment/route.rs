use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;


use crate::app::AppState;
use crate::routes::AttachmentResponse;
use crate::routes::FailedResponse;


pub fn attachment_route(state: AppState) -> Router {
    Router::new()
        .route("/:att_id", get(find_attachment_content_by_id))
        .with_state(state)
}

pub async fn find_attachment_content_by_id(
    Path(att_id): Path<i32>,
    State(state): State<AppState>
) -> Response {
    let res = state.attachment_service
        .find_attachment_image_by_id(att_id)
        .await;
    let (att, att_type) = match res {
        Ok(r) => r,
        Err(e) => return FailedResponse(e).into_response()
    };
    AttachmentResponse(att, att_type).into_response()
}