use actix_web::{HttpServer, middleware::Logger, App, web};
use actix_cors::Cors;
use log::info;
use crate::routes::healthcheck_route::healthcheck;
use crate::routes::auth_route::auth_config;

pub async fn run() -> std::io::Result<()> {

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin();

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .service(web::scope("/healthcheck").service(healthcheck))
            .service(web::scope("/auth").configure(auth_config))
    });

    let port = 8080;
    info!("Server will be running on port {port}");
    server
        .bind(("0.0.0.0", port))?
        .run()
        .await?;

    Ok(())
}