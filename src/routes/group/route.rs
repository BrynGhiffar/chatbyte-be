use axum::extract::Path;
use axum::extract::State;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

use crate::app::AppState;
use crate::routes::AuthorizedUser;
use crate::routes::ImageResponse;
use crate::routes::ServerResponse;
use crate::routes::ServerResponse::*;
use crate::service::CreateGroupForm;
use crate::service::GroupModel;

pub fn group_route(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_group))
        .route("/image/:group_id", get(find_group_profile))
        .with_state(state)
}

async fn create_group(
    AuthorizedUser { user_id }: AuthorizedUser,
    State(state): State<AppState>,
    form: CreateGroupForm,
) -> ServerResponse<GroupModel> {
    let res = state.group_service.create_group(user_id, form).await;
    match res {
        Ok(res) => Success(res),
        Err(e) => Failed(e),
    }
}

async fn find_group_profile(
    Path(group_id): Path<i32>,
    State(state): State<AppState>,
) -> ImageResponse {
    ImageResponse(state.group_service.find_group_profile_image(group_id).await)
}
