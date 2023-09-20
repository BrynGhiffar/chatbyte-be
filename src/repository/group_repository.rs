use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use serde::Serialize;

#[derive(Clone)]
pub struct GroupRepository {
    conn: Pool<Postgres>
}

impl GroupRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        return GroupRepository { conn };
    }

    pub async fn create_group(&self, name: String) -> Result<Group, String> {
        let result = sqlx::query_as::<_, Group>("INSERT INTO PUBLIC.GROUP (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(&self.conn).await;
        return result.map_err(|e| e.to_string());
    }

    pub async fn rename_group(&self, group_id: i32, new_name: String) -> Result<Group, String> {
        let result = sqlx::query_as::<_, Group>("UPDATE PUBLIC.GROUP SET name = $2 WHERE id = $1")
            .bind(group_id)
            .bind(new_name)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(result);
    }

    pub async fn add_user_to_group(&self, user_id: i32, group_id: i32) -> Result<bool, String> {
        let result = sqlx::query("INSERT INTO PUBLIC.GROUP_MEMBER (group_id, user_id) VALUES ($1, $2)")
            .bind(group_id)
            .bind(user_id)
            .execute(&self.conn).await
            .map_err(|e| e.to_string())?;
        return Ok(result.rows_affected() == 1); 
    }

    pub async fn remove_user_from_group(&self, user_id: i32, group_id: i32) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM PUBLIC.GROUP_MEMBER WHERE user_id = $1 and group_id = $2")
            .bind(user_id)
            .bind(group_id)
            .execute(&self.conn).await
            .map_err(|e| e.to_string())?;
        return Ok(result.rows_affected() == 1);
    }

    pub async fn delete_group(&self, group_id: i32) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM PUBLIC.GROUP WHERE group_id = $1")
            .bind(group_id)
            .execute(&self.conn).await
            .map_err(|e| e.to_string())?;

        return Ok(result.rows_affected() == 1);
    }

    pub async fn find_groups_for_user(&self, user_id: i32) -> Result<Vec<Group>, String> {
        let result = sqlx::query_as::<_, Group>("
        SELECT
            PUBLIC.GROUP.ID AS id, PUBLIC.GROUP.NAME AS name
        FROM PUBLIC.GROUP_MEMBER 
        JOIN PUBLIC.GROUP ON PUBLIC.GROUP.ID = PUBLIC.GROUP_MEMBER.GROUP_ID 
        WHERE PUBLIC.GROUP_MEMBER.USER_ID = $1
        ")
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())?;

        return Ok(result);
    }

    pub async fn get_profile_image_for_group(&self, group_id: i32) -> Result<Option<GroupImage>, String> {
        let result = sqlx::query_as::<_,GroupImage>("
            SELECT PUBLIC.GROUP_AVATAR.GROUP_IMAGE FROM PUBLIC.GROUP_AVATAR WHERE PUBLIC.GROUP_AVATAR.GROUP_ID = $1
        ")
            .bind(group_id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(result);
    }

    pub async fn set_profile_image_for_group(&self, group_id: i32, group_image: Vec<u8>) -> Result<bool, String> {
        let result = sqlx::query("
            INSERT INTO PUBLIC.GROUP_AVATAR (group_id, group_image) VALUES ($1, $2) 
                ON CONFLICT (group_id) DO
                UPDATE
                    SET group_image = $2
            ")
            .bind(group_id)
            .bind(group_image)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(result.rows_affected() == 1);
    }

    pub async fn create_group_message(
        &self, 
        group_id: i32, 
        sender_id: i32, 
        content: String
    )  -> Result<GroupMessage, String> {
        let result = sqlx::query_as::<_,GroupMessage>("
                INSERT INTO PUBLIC.GROUP_MESSAGE (group_id, sender_id, content)
                VALUES ($1, $2, $3)
                RETURNING *
            ")
            .bind(group_id)
            .bind(sender_id)
            .bind(content)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(result);
    }

    pub async fn find_group_members(&self, group_id: i32) -> Result<Vec<i32>, String> {
        let result: Vec<(i32,)> = sqlx::query_as("SELECT user_id FROM PUBLIC.GROUP_MEMBER WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        
        return Ok(result.iter().map(|t| t.0).collect());
    }

    pub async fn find_all_group_message(&self, group_id: i32) -> Result<Vec<GroupMessage>, String> {

        let result = sqlx::query_as::<_,GroupMessage>("
                SELECT * FROM PUBLIC.GROUP_MESSAGE WHERE group_id = $1
            ")
            .bind(group_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(result);
    }

}

#[derive(sqlx::FromRow, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String
}

#[derive(sqlx::FromRow, Serialize)]
pub struct GroupMessage {
    pub id: i32,
    pub sender_id: i32,
    pub group_id: i32,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub deleted: bool,
}

#[derive(sqlx::FromRow)]
pub struct GroupImage(pub Vec<u8>);