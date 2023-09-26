use chrono::{NaiveDateTime, Local};
use sqlx::{Pool, Postgres, postgres::PgRow, Row, Error, FromRow};
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
        let sent_at = Local::now().naive_local();
        sqlx::query_as::<_,GroupMessage>("
            WITH GM AS (
                INSERT INTO PUBLIC.GROUP_MESSAGE (GROUP_ID, SENDER_ID, CONTENT, SENT_AT) VALUES($1, $2, $3, $4) RETURNING *
            ) SELECT     
                GM.ID as ID,
                GM.SENDER_ID AS SENDER_ID,
                U.USERNAME AS USERNAME,
                GM.CONTENT AS CONTENT,
                GM.GROUP_ID AS GROUP_ID,
                GM.DELETED AS DELETED,
                GM.SENT_AT AS SENT_AT
            FROM GM JOIN PUBLIC.USER U ON U.ID = GM.SENDER_ID;
            ")
            .bind(group_id)
            .bind(sender_id)
            .bind(content)
            .bind(sent_at)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
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
            SELECT 
                GM.ID as ID,
                GM.SENDER_ID AS SENDER_ID,
                U.USERNAME AS USERNAME,
                GM.CONTENT AS CONTENT,
                GM.GROUP_ID AS GROUP_ID,
                GM.DELETED AS DELETED,
                GM.SENT_AT AS SENT_AT
            FROM PUBLIC.GROUP_MESSAGE  GM
                JOIN PUBLIC.USER U ON U.ID = GM.SENDER_ID
                WHERE GROUP_ID = $1;
            ")
            .bind(group_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())?;
        return Ok(result);
    }

    pub async fn find_user_group_recent(&self, user_id: i32) -> Result<Vec<GroupConversation>, String> {
        sqlx::query_as::<_, GroupConversation>("
            SELECT * FROM 
            (
                SELECT GMEM.GROUP_ID,
                    (SUM(
                        CASE WHEN GM.ID IS NOT NULL THEN 1 ELSE 0 END
                    ) - 
                    SUM(
                        CASE WHEN GMR.READER_ID IS NOT NULL THEN 1 ELSE 0 END
                    )) AS UNREAD_MESSAGE
                FROM PUBLIC.GROUP_MEMBER GMEM
                    LEFT JOIN PUBLIC.GROUP_MESSAGE GM ON GM.GROUP_ID = GMEM.GROUP_ID
                    LEFT JOIN PUBLIC.GROUP_MESSAGE_READ GMR 
                        ON GMR.GROUP_ID = GMEM.GROUP_ID 
                            AND GMR.READER_ID = GMEM.USER_ID
                            AND GMR.MESSAGE_ID = GM.ID
                WHERE GMEM.USER_ID = $1
                GROUP BY GMEM.GROUP_ID
            ) UM
                JOIN PUBLIC.LAST_MESSAGE_GROUP LMG ON UM.GROUP_ID = LMG.GROUP_ID
            ;
        ")
        .bind(user_id)
        .fetch_all(&self.conn)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn read_all_message(&self, user_id: i32, group_id: i32) -> Result<bool, String> {
        sqlx::query("
            INSERT INTO PUBLIC.GROUP_MESSAGE_READ(message_id, reader_id, group_id)
            SELECT GM.id as message_id, $1 as reader_id, GM.GROUP_ID FROM PUBLIC.GROUP_MESSAGE GM
            WHERE GM.GROUP_ID = $2
            AND
            NOT EXISTS (
                SELECT 1 FROM PUBLIC.GROUP_MESSAGE_READ GMR 
                    WHERE GMR.message_id = GM.id 
                        AND GMR.reader_id = $1
                        AND GMR.group_id = $2
            );
        ")
        .bind(user_id)
        .bind(group_id)
        .execute(&self.conn)
        .await
        .map_err(|e| e.to_string())
        .map(|_| true)
    }

}

#[derive(sqlx::FromRow, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupConversationDetail {
    pub username: String,
    pub sent_at: String,
    pub content: String,
    pub deleted: bool    
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupConversation {
    pub group_id: i32,
    pub unread_message: i64,
    pub group_name: String,
    pub detail: Option<GroupConversationDetail>
}

impl FromRow<'_, PgRow> for GroupConversation {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let content = row.try_get::<Option<String>, _>("content")?;
        let detail = if let Some(content) = content {
            let sent_at: NaiveDateTime = row.try_get("sent_at")?;
            Some(GroupConversationDetail {
                content,
                username: row.try_get("username")?,
                deleted: row.try_get("deleted")?,
                sent_at: sent_at.format("%H:%M").to_string(),
            })
        } else { None };
        Ok(Self {
            group_id: row.try_get("group_id")?,
            unread_message: row.try_get("unread_message")?,
            group_name: row.try_get("group_name")?,
            detail
        })
    }
}

#[derive(sqlx::FromRow, Serialize)]
pub struct GroupMessage {
    pub id: i32,
    pub sender_id: i32,
    pub username: String,
    pub group_id: i32,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub deleted: bool,
}

#[derive(sqlx::FromRow)]
pub struct GroupImage(pub Vec<u8>);