use chrono::Local;
use sqlx::Pool;
use sqlx::Postgres;

use super::Session;
use super::CREATE_SESSION_STMT;
use super::FIND_SESSION_BY_USER_ID;

#[derive(Clone)]
pub struct SessionRepository {
    conn: Pool<Postgres>,
}

impl SessionRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        SessionRepository { conn }
    }

    pub async fn create_session(
        &self,
        user_id: i32,
        operating_system: Option<String>,
        agent: Option<String>,
    ) -> Result<Session, String> {
        let now = Local::now().naive_local();
        sqlx::query_as::<_, Session>(CREATE_SESSION_STMT)
            .bind(user_id)
            .bind(operating_system)
            .bind(agent)
            .bind(now)
            .bind(now)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_sessions_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Vec<Session>, String> {
        sqlx::query_as::<_, Session>(FIND_SESSION_BY_USER_ID)
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}
