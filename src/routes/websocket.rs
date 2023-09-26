use actix_web::{
    web::{Data, Payload},
    Error, HttpRequest, HttpResponse,
    web
};
use tokio::task::spawn_local;
use serde::Deserialize;

use crate::{app::AppState, middleware::verify_token, utility::bad_request};

pub async fn websocket(
    req: HttpRequest,
    stream: Payload,
    app: Data<AppState>,
    query: web::Query<WebsocketQuery>,
) -> Result<HttpResponse, Error> {
    let (res, ws_tx, ws_rx) = actix_ws::handle(&req, stream)?;
    let WebsocketQuery { token } = query.into_inner();
    let uid = match verify_token(token.clone()) {
        Err(e) => return Ok(bad_request(e)),
        Ok(uid) => uid
    };
    let session = app.session_factory.create_session(token, uid, ws_tx);
    spawn_local(session.run(ws_rx));
    Ok(res)
}

// --- UTILITY SRUCTS ---
#[derive(Deserialize)]
pub struct WebsocketQuery {
    token: String,
}