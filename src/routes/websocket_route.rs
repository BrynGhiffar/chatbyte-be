use actix_web::{
    web::{Data, Payload},
    Error, HttpRequest, HttpResponse,
};
use tokio::task::spawn_local;

use crate::{app::AppState, middleware::get_uid_from_header};

pub async fn websocket(
    req: HttpRequest,
    stream: Payload,
    app: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (res, ws_tx, ws_rx) = actix_ws::handle(&req, stream)?;
    let uid = get_uid_from_header(req.clone()).unwrap();
    let token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let session = app.session_factory.create_session(token, uid, ws_tx);
    spawn_local(session.run(ws_rx));
    Ok(res)
}
