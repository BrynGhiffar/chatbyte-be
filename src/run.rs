use std::env;
use std::net::SocketAddr;

use crate::app::AppState;
use crate::routes;
use crate::routes::attachment_route;
use crate::routes::auth_route;
use crate::routes::contact_route;
use crate::routes::group_route;
use crate::routes::message_route;
use crate::routes::user_route;
use crate::routes::ws_route;
use axum::routing::get;
use axum::Router;
use futures_util::join;
use tokio::spawn;
use tower_http::cors::CorsLayer;

pub async fn axum_run() {
    let (state, ws_server) = AppState::default().await;
    let ws_server = spawn(ws_server.run());
    let port: u16 = env::var("PORT")
        .expect("PORT environment variable undefined")
        .parse()
        .expect("Expect port number to be an integer");

    let app = Router::new()
        .route("/api/healthcheck", get(routes::healthcheck))
        .nest("/api/auth", auth_route(state.clone()))
        .nest("/api/contact", contact_route(state.clone()))
        .nest("/api/message", message_route(state.clone()))
        .nest("/api/group", group_route(state.clone()))
        .nest("/api/user", user_route(state.clone()))
        .nest("/api/attachment", attachment_route(state.clone()))
        .nest("/api/ws", ws_route(state.clone()))
        .layer(CorsLayer::permissive());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    log::info!("Chatbyte BE running on http://0.0.0.0:{port}");
    log::info!("Healthcheck: http://0.0.0.0:{port}/api/healthcheck");

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let _ = join!(ws_server, server);
}
