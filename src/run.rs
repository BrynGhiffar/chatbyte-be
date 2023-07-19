use actix_web::{HttpServer, middleware::Logger, App, web};
use actix_cors::Cors;
use crate::app::AppState;
use crate::routes::contact_route::contact_config;
use crate::routes::healthcheck_route::healthcheck;
use crate::routes::auth_route::auth_config;
use crate::routes::message_route::message_config;
use crate::routes::user_route::user_config;

pub async fn run() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let state = AppState::default().await;

    let server = HttpServer::new(move || {

        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:5173")
        //     .allow_any_origin()
        //     .allow_any_header();
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