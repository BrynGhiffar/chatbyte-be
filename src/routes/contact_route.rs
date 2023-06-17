use actix_web::{
    web::{self, ServiceConfig},
    Responder, HttpRequest,
};
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect, QueryFilter, ColumnTrait};
use serde::Serialize;

use crate::{
    app::AppState,
    entities::user,
    utility::{server_error, success}, middleware::{VerifyToken, get_uid_from_header},
};

#[derive(FromQueryResult, Serialize)]
struct Contact {
    id: i32,
    email: String,
    username: String,
}

async fn get_contacts(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let db = &state.db;
    let uid = get_uid_from_header(req).unwrap();
    let contacts = user::Entity::find()
        .filter(user::Column::Id.ne(uid))
        .select_only()
        .column(user::Column::Id)
        .column(user::Column::Email)
        .column(user::Column::Username)
        .into_model::<Contact>()
        .all(db)
        .await;
    let Some(contacts) = contacts.ok() else {
        return server_error("A database error occurred");
    };

    return success(contacts);
}

pub fn contact_config(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_contacts).wrap(VerifyToken));
}
