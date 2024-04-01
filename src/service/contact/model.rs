use crate::repository::ContactRepositoryModel;
use crate::repository::ConversationRecentMessageRepositoryModel;
use crate::repository::GroupConversationDetailRepositoryModel;
use crate::repository::GroupConversationRepositoryModel;
use crate::repository::GroupRepositoryModel;

use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct DirectContact {
    pub id: i32,
    pub email: String,
    pub username: String,
}

impl From<&ContactRepositoryModel> for DirectContact {
    fn from(value: &ContactRepositoryModel) -> Self {
        Self {
            id: value.id,
            email: value.email.clone(),
            username: value.username.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectConversation {
    pub id: i32,
    pub contact_id: i32,
    pub last_message: String,
    pub unread_count: i64,
    pub username: String,
    pub deleted: bool,
    pub sent_at: NaiveDateTime,
}

impl From<&ConversationRecentMessageRepositoryModel> for DirectConversation {
    fn from(value: &ConversationRecentMessageRepositoryModel) -> Self {
        Self {
            id: value.id,
            contact_id: value.contact_id,
            last_message: value.last_message.clone(),
            unread_count: value.unread_count,
            username: value.username.clone(),
            deleted: value.deleted,
            sent_at: value.sent_at.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct GroupContact {
    id: i32,
    name: String,
}

impl From<GroupRepositoryModel> for GroupContact {
    fn from(GroupRepositoryModel { id, name }: GroupRepositoryModel) -> Self {
        GroupContact { id, name }
    }
}

impl From<&GroupRepositoryModel> for GroupContact {
    fn from(value: &GroupRepositoryModel) -> Self {
        let model = value.clone();
        GroupContact::from(model)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupConversation {
    pub group_id: i32,
    pub unread_message: i64,
    pub group_name: String,
    pub detail: Option<GroupConversationDetail>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupConversationDetail {
    pub username: String,
    pub sent_at: NaiveDateTime,
    pub content: String,
    pub deleted: bool,
}

impl From<GroupConversationDetailRepositoryModel> for GroupConversationDetail {
    fn from(
        GroupConversationDetailRepositoryModel {
            username,
            sent_at,
            content,
            deleted,
        }: GroupConversationDetailRepositoryModel
    ) -> Self {
        Self {
            username,
            sent_at,
            content,
            deleted,
        }
    }
}

impl From<&GroupConversationRepositoryModel> for GroupConversation {
    fn from(value: &GroupConversationRepositoryModel) -> Self {
        GroupConversation::from(value.clone())
    }
}

impl From<GroupConversationRepositoryModel> for GroupConversation {
    fn from(
        GroupConversationRepositoryModel {
            group_id,
            unread_message,
            group_name,
            detail,
        }: GroupConversationRepositoryModel
    ) -> Self {
        Self {
            group_id,
            unread_message,
            group_name,
            detail: detail.map(GroupConversationDetail::from),
        }
    }
}
