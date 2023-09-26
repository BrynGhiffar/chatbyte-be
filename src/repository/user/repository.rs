use sqlx::{Pool, Postgres};
use super::{USER_PROFILE_UPSERT_STATEMENT, UserAvatar, GET_USER_PROFILE};

#[derive(Clone)]
pub struct UserRepository {
    conn: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        UserRepository { conn }
    }

    pub async fn upsert_user_profile(&self, user_id: i32, image: Vec<u8>) -> Result<bool, String> {

        sqlx::query(USER_PROFILE_UPSERT_STATEMENT)
            .bind(user_id)
            .bind(image)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn get_avatar(&self, user_id: i32) -> Result<Option<UserAvatar>, String> {
        sqlx::query_as::<_, UserAvatar>(GET_USER_PROFILE)
            .bind(user_id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}
