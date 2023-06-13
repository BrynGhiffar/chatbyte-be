mod run;
mod routes;
mod res_model;
mod req_model;
use actix_web;
mod entities;
mod app;
mod middleware;
mod utility;
mod message;
// use entities::{ prelude::*, * };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run::run().await
}