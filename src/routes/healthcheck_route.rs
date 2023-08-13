use actix_web::{Responder, web::Data};

use crate::app::AppState;

pub async fn healthcheck(state: Data<AppState>) -> impl Responder {
    let tx = state.transmitter.clone();
    tx.send("Health check".to_string()).unwrap();
    "API is healthy"
}
