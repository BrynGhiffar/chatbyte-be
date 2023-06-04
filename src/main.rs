mod run;
mod routes;
mod res_model;
mod req_model;
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run::run().await
}