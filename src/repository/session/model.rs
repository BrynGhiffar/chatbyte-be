use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct Session {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub last_active: NaiveDateTime,
    pub operating_system: Option<String>,
    pub agent: Option<String>,
    pub user_id: i32,
}