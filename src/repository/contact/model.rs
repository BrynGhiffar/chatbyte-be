use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct ContactRepositoryModel {
    pub id: i32,
    pub email: String,
    pub username: String,
}
