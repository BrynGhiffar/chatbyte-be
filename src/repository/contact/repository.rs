use sqlx::{Pool, Postgres};

use super::{GET_CONTACT_STMT, Contact};

#[derive(Clone)]
pub struct ContactRepository {
    conn: Pool<Postgres>,
}

impl ContactRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        ContactRepository { conn }
    }

    pub async fn get_contacts(&self, user_id: i32) -> Result<Vec<Contact>, String> {
        sqlx::query_as::<_, Contact>(GET_CONTACT_STMT)
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}