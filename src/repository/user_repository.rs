use sea_orm::{DatabaseConnection, ConnectionTrait, Statement, DatabaseBackend::*, DbErr, EntityTrait, ColumnTrait, QueryFilter};

use crate::entities::user_avatar;


#[derive(Clone)]
pub struct UserRepository {
    conn: DatabaseConnection
}

impl UserRepository {
    pub fn new(conn: DatabaseConnection) -> Self { 
        UserRepository { conn }
    }

    pub async fn upsert_user_profile(&self, user_id: i32, image: Vec<u8>) -> Result<bool, DbErr> {
        self.conn.execute(Statement::from_sql_and_values(
            Postgres, r#"INSERT INTO public.user_avatar (user_id, avatar_image) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET avatar_image = $2"#,
        [user_id.into(), image.into()]))
        .await?;
        Ok(true)
    }

    pub async fn get_avatar(&self, user_id: i32) -> Result<Option<user_avatar::Model>, DbErr> {
        user_avatar::Entity::find().filter(user_avatar::Column::UserId.eq(user_id)).one(&self.conn).await
    }
}