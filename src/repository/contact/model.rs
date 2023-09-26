use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Contact {
    id: i32,
    email: String,
    username: String,
}
