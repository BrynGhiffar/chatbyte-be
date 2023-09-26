use sqlx::{Pool, Postgres};

use super::{User, FIND_USER_BY_EMAIL_STMT, FIND_USER_BY_ID_STMT, UPDATE_USERNAME_STMT, UPDATE_PASSWORD_STMT, UPDATE_EMAIL_STMT, CREATE_USER_STMT};

#[derive(Clone)]
pub struct AuthRepository {
    conn: Pool<Postgres>,
}

impl AuthRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        AuthRepository { conn }
    }

    pub async fn find_user_by_email(&self, email: String) -> Result<Option<User>, String> {
        sqlx::query_as::<_, User>(FIND_USER_BY_EMAIL_STMT)
            .bind(email)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_user_by_id(&self, uid: i32) -> Result<Option<User>, String> {
        sqlx::query_as::<_, User>(FIND_USER_BY_ID_STMT)
            .bind(uid)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_username(&self, uid: i32, username: String) -> Result<bool, String> {
        sqlx::query(UPDATE_USERNAME_STMT)
            .bind(username)
            .bind(uid)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn update_email(&self, uid: i32, email: String) -> Result<bool, String> {
        sqlx::query(UPDATE_EMAIL_STMT)
            .bind(email)
            .bind(uid)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn update_password(&self, uid: i32, password: String) -> Result<bool, String> {
        sqlx::query(UPDATE_PASSWORD_STMT)
            .bind(password)
            .bind(uid)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn create_user(&self, email: String, password: String) -> Result<bool, String> {
        sqlx::query(CREATE_USER_STMT)
            .bind(email)
            .bind(password)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }
}
