use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::Error;
use sqlx::FromRow;
use sqlx::Row;

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct GroupRepositoryModel {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupConversationDetailRepositoryModel {
    pub username: String,
    pub sent_at: NaiveDateTime,
    pub content: String,
    pub deleted: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupConversationRepositoryModel {
    pub group_id: i32,
    pub unread_message: i64,
    pub group_name: String,
    pub detail: Option<GroupConversationDetailRepositoryModel>,
}

impl FromRow<'_, PgRow> for GroupConversationRepositoryModel {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let content = row.try_get::<Option<String>, _>("content")?;
        let detail = if let Some(content) = content {
            let sent_at: NaiveDateTime = row.try_get("sent_at")?;
            let deleted: bool = row.try_get("deleted")?;
            let content = if deleted { String::from("") } else { content };
            Some(GroupConversationDetailRepositoryModel {
                content,
                username: row.try_get("username")?,
                deleted,
                sent_at,
            })
        } else {
            None
        };
        Ok(Self {
            group_id: row.try_get("group_id")?,
            unread_message: row.try_get("unread_message")?,
            group_name: row.try_get("group_name")?,
            detail,
        })
    }
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct GroupMessageRepositoryModel {
    pub id: i32,
    pub sender_id: i32,
    pub username: String,
    pub group_id: i32,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub edited: bool,
    pub deleted: bool,
}

#[derive(sqlx::FromRow)]
pub struct GroupImage(pub Vec<u8>);
