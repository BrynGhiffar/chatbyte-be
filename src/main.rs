mod req_model;
mod routes;
mod run;
use actix_web;
mod app;
mod message;
mod middleware;
mod repository;
mod utility;
// use entities::{ prelude::*, * };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run::run().await
}
