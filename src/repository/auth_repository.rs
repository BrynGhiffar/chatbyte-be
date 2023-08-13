use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend,
    DatabaseConnection, DbErr, EntityTrait, QueryFilter, Statement,
};

use crate::entities::user;

#[derive(Clone)]
pub struct AuthRepository {
    conn: DatabaseConnection,
}

impl AuthRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        AuthRepository { conn }
    }

    pub async fn find_user_by_email(&self, email: String) -> Result<Option<user::Model>, DbErr> {
        let tuser = user::Entity::find()
            .filter(user::Column::Email.eq(email.clone()))
            .one(&self.conn)
            .await?;
        Ok(tuser)
    }

    pub async fn update_username(&self, uid: i32, username: String) -> bool {
        let Ok(user) = user::Entity::find_by_id(uid).one(&self.conn).await else { return false; };
        let Some(user) = user else { return false; };
        let mut user: user::ActiveModel = user.into();
        user.username = ActiveValue::Set(username);
        let Ok(_) = user.update(&self.conn).await else { return false; };
        return true;
    }

    pub async fn update_email(&self, uid: i32, email: String) -> bool {
        let Ok(user) = user::Entity::find_by_id(uid).one(&self.conn).await else { return false; };
        let Some(user) = user else { return false; };
        let mut user: user::ActiveModel = user.into();
        user.email = ActiveValue::Set(email);
        let Ok(_) = user.update(&self.conn).await else { return false; };
        return true;
    }

    pub async fn update_password(&self, uid: i32, password: String) -> bool {
        let Some(exec_res) = self.conn
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres, r#"
                    UPDATE public.user SET password = crypt($1, gen_salt('bf', 5)) where id = $2
                "#, [password.into(), uid.into()]))
            .await.ok() else { return false; };
        if exec_res.rows_affected() == 0 {
            return false;
        };
        return true;
    }
}
