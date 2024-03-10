#[derive(sqlx::FromRow)]
pub struct UserModelRepository {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}
