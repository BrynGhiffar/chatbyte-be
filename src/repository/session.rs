use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

use super::entities::session;

#[derive(Clone)]
pub struct SessionRepository {
    conn: DatabaseConnection,
}

impl SessionRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        SessionRepository { conn }
    }

    pub async fn create_session(
        &self,
        user_id: i32,
        operating_system: Option<String>,
        agent: Option<String>,
    ) -> Result<session::Model, DbErr> {
        let model = session::ActiveModel {
            user_id: ActiveValue::Set(user_id),
            operating_system: ActiveValue::Set(operating_system),
            agent: ActiveValue::Set(agent),
            ..Default::default()
        };
        model.insert(&self.conn).await
    }

    pub async fn find_sessions_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Vec<session::Model>, DbErr> {
        let query = session::Entity::find()
            .filter(session::Column::UserId.eq(user_id))
            .all(&self.conn);
        query.await
    }
}
