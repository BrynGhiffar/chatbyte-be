use std::net::SocketAddr;

use crate::actix_routes::auth_config;
use crate::actix_routes::contact_config;
use crate::actix_routes::group_config;
use crate::actix_routes::healthcheck;
use crate::actix_routes::message_config;
use crate::actix_routes::user_config;
use crate::actix_routes::websocket;
use crate::app::AppState;
use crate::routes;
use crate::routes::attachment_route;
use crate::routes::auth_route;
use crate::routes::contact_route;
use crate::routes::group_route;
use crate::routes::message_route;
use crate::routes::user_route;
use crate::routes::ws_route;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use axum::routing::get;
use axum::Router;
use futures_util::join;
use tokio::spawn;
use tokio::try_join;
use tower_http::cors::CorsLayer;

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let (state, ws_server) = AppState::default().await;
    let ws_server = spawn(ws_server.run());

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .route("/api/healthcheck", web::get().to(healthcheck))
            .service(web::scope("/api/auth").configure(auth_config))
            .service(web::scope("/api/message").configure(message_config))
            .service(web::scope("/api/contacts").configure(contact_config))
            .service(web::scope("/api/user").configure(user_config))
            .service(web::scope("/api/group").configure(group_config))
            .service(web::resource("/api/ws").route(web::get().to(websocket)))
    });

    let port = 8080;
    log::info!("Server will be running on port {port}");
    let http_server = server.workers(2).bind(("0.0.0.0", port))?.run();

    try_join!(http_server, async move { ws_server.await.unwrap() })?;

    Ok(())
}

pub async fn axum_run() {
    let (state, ws_server) = AppState::default().await;
    let ws_server = spawn(ws_server.run());



    let app = Router::new()
        .route("/api/healthcheck", get(routes::healthcheck))
        .nest("/api/auth", auth_route(state.clone()))
        .nest("/api/contact", contact_route(state.clone()))
        .nest("/api/message", message_route(state.clone()))
        .nest("/api/group", group_route(state.clone()))
        .nest("/api/user", user_route(state.clone()))
        .nest("/api/attachment", attachment_route(state.clone()))
        .nest("/api/ws", ws_route(state.clone()))
        .layer(CorsLayer::permissive())
        ;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    
    let _ = join!(ws_server, server);
}
