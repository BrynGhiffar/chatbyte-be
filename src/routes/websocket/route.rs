use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::app::AppState;
use crate::routes::AuthorizedUserFromTokenQuery;
use crate::routes::TokenQuery;

pub fn ws_route(state: AppState) -> Router {
    Router::new().route("/", get(ws_handler)).with_state(state)
}

pub async fn ws_handler(
    TokenQuery { token }: TokenQuery,
    AuthorizedUserFromTokenQuery { user_id }: AuthorizedUserFromTokenQuery,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let session = state.session_factory.create_session(token, user_id);
    ws.on_upgrade(move |socket| session.run(socket))
}
