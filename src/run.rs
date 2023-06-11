use actix_web::{HttpServer, middleware::Logger, App, web};
use actix_cors::Cors;
use crate::app::AppState;
use crate::routes::contact_route::contact_config;
use crate::routes::healthcheck_route::healthcheck;
use crate::routes::auth_route::auth_config;
use crate::routes::message_route::message_config;

pub async fn run() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let state = AppState::default().await;

    let server = HttpServer::new(move || {

        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(web::Data::new(state.clone()))
            .route("/healthcheck", web::get().to(healthcheck))
            .service(web::scope("/auth").configure(auth_config))
            .service(web::scope("/message").configure(message_config))
            .service(web::scope("/contact").configure(contact_config))
    });

    let port = 8080;
    log::info!("Server will be running on port {port}");
    server
        .workers(2)
        .bind(("0.0.0.0", port))?
        .run()
        .await?;

    Ok(())
}