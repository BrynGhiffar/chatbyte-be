use crate::routes::auth_config;
use crate::routes::contact_config;
use crate::routes::group_config;
use crate::routes::healthcheck;
use crate::routes::message_config;
use crate::routes::user_config;
use crate::{app::AppState, routes::websocket};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tokio::{spawn, try_join};

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
