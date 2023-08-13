use crate::entities::user;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};
use serde::Serialize;

#[derive(Clone)]
pub struct ContactRepository {
    conn: DatabaseConnection,
}

impl ContactRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        ContactRepository { conn }
    }

    pub async fn get_contacts(&self, user_id: i32) -> Result<Vec<Contact>, DbErr> {
        let contacts = user::Entity::find()
            .filter(user::Column::Id.ne(user_id))
            .select_only()
            .column(user::Column::Id)
            .column(user::Column::Email)
            .column(user::Column::Username)
            .into_model::<Contact>()
            .all(&self.conn)
            .await?;
        Ok(contacts)
    }
}

#[derive(FromQueryResult, Serialize)]
pub struct Contact {
    id: i32,
    email: String,
    username: String,
}
