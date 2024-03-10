use sqlx::Pool;
use sqlx::Postgres;

use super::ContactRepositoryModel;
use super::GET_CONTACT_STMT;

#[derive(Clone)]
pub struct ContactRepository {
    conn: Pool<Postgres>,
}

impl ContactRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        ContactRepository { conn }
    }

    pub async fn get_contacts(&self, user_id: i32) -> Result<Vec<ContactRepositoryModel>, String> {
        sqlx::query_as::<_, ContactRepositoryModel>(GET_CONTACT_STMT)
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}
