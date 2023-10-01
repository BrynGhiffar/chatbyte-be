use chrono::Local;

use sqlx::{Pool, Postgres, Executor};

use super::{
    Group, GroupConversationRepositoryModel, GroupImage, GroupMessageRepositoryModel, ADD_USER_TO_GROUP_STMT,
    CREATE_GROUP_MESSAGE_STMT, CREATE_GROUP_STMT, DELETE_GROUP_STMT, FIND_ALL_GROUP_MESSAGE_STMT,
    FIND_GROUP_FOR_USER_STMT, FIND_GROUP_MEMBER_STMT, FIND_USER_GROUP_RECENT_STMT,
    GET_PROFILE_IMAGE_FOR_GROUP_STMT, READ_ALL_MESSAGE_STMT, REMOVE_USER_FROM_GROUP_STMT,
    RENAME_GROUP_STMT, SET_PROFILE_IMAGE_FOR_GROUP_STMT, SET_MESSAGE_DELETE_STMT, FIND_GROUP_MESSAGE_BY_ID, EDIT_MESSAGE_BY_ID_STMT,
};

#[derive(Clone)]
pub struct GroupRepository {
    conn: Pool<Postgres>,
}

impl GroupRepository {
    pub fn new(conn: Pool<Postgres>) -> Self {
        return GroupRepository { conn };
    }

    pub async fn create_group(&self, name: String) -> Result<Group, String> {
        sqlx::query_as::<_, Group>(CREATE_GROUP_STMT)
            .bind(name)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn rename_group(&self, group_id: i32, new_name: String) -> Result<Group, String> {
        sqlx::query_as::<_, Group>(RENAME_GROUP_STMT)
            .bind(group_id)
            .bind(new_name)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn add_user_to_group(&self, user_id: i32, group_id: i32) -> Result<bool, String> {
        sqlx::query(ADD_USER_TO_GROUP_STMT)
            .bind(group_id)
            .bind(user_id)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn remove_user_from_group(
        &self,
        user_id: i32,
        group_id: i32,
    ) -> Result<bool, String> {
        sqlx::query(REMOVE_USER_FROM_GROUP_STMT)
            .bind(user_id)
            .bind(group_id)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn delete_group(&self, group_id: i32) -> Result<bool, String> {
        sqlx::query(DELETE_GROUP_STMT)
            .bind(group_id)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn find_groups_for_user(&self, user_id: i32) -> Result<Vec<Group>, String> {
        sqlx::query_as::<_, Group>(FIND_GROUP_FOR_USER_STMT)
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_profile_image_for_group(
        &self,
        group_id: i32,
    ) -> Result<Option<GroupImage>, String> {
        sqlx::query_as::<_, GroupImage>(GET_PROFILE_IMAGE_FOR_GROUP_STMT)
            .bind(group_id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn set_profile_image_for_group(
        &self,
        group_id: i32,
        group_image: Vec<u8>,
    ) -> Result<bool, String> {
        sqlx::query(SET_PROFILE_IMAGE_FOR_GROUP_STMT)
            .bind(group_id)
            .bind(group_image)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn create_group_message(
        &self,
        group_id: i32,
        sender_id: i32,
        content: String,
    ) -> Result<GroupMessageRepositoryModel, String> {
        Self::create_group_message_with_executor(
            &self.conn, 
            group_id, 
            sender_id, 
            content
        ).await
    }

    pub async fn create_group_message_with_executor<'a, T>(
        exec: T,
        group_id: i32,
        sender_id: i32,
        content: String,
    ) -> Result<GroupMessageRepositoryModel, String>
        where T: Executor<'a, Database = Postgres> 
    {
        let sent_at = Local::now().naive_local();
        sqlx::query_as::<_, GroupMessageRepositoryModel>(CREATE_GROUP_MESSAGE_STMT)
            .bind(group_id)
            .bind(sender_id)
            .bind(content)
            .bind(sent_at)
            .fetch_one(exec)
            .await
            .map_err(|e| e.to_string())

    }

    pub async fn find_group_members(&self, group_id: i32) -> Result<Vec<i32>, String> {
        sqlx::query_as::<_, (i32,)>(FIND_GROUP_MEMBER_STMT)
            .bind(group_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.iter().map(|t| t.0).collect())
    }

    pub async fn find_all_group_message(&self, group_id: i32) -> Result<Vec<GroupMessageRepositoryModel>, String> {
        sqlx::query_as::<_, GroupMessageRepositoryModel>(FIND_ALL_GROUP_MESSAGE_STMT)
            .bind(group_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_user_group_recent(
        &self,
        user_id: i32,
    ) -> Result<Vec<GroupConversationRepositoryModel>, String> {
        sqlx::query_as::<_, GroupConversationRepositoryModel>(FIND_USER_GROUP_RECENT_STMT)
            .bind(user_id)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn read_all_message(&self, user_id: i32, group_id: i32) -> Result<bool, String> {
        sqlx::query(READ_ALL_MESSAGE_STMT)
            .bind(user_id)
            .bind(group_id)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|_| true)
    }

    pub async fn set_message_to_delete(&self, message_id: i32) -> Result<bool, String> {
        sqlx::query(SET_MESSAGE_DELETE_STMT)
            .bind(message_id)
            .execute(&self.conn)
            .await
            .map_err(|e| e.to_string())
            .map(|r| r.rows_affected() == 1)
    }

    pub async fn find_message_by_id(&self, message_id: i32) -> Result<Option<GroupMessageRepositoryModel>, String> {
        sqlx::query_as::<_, GroupMessageRepositoryModel>(FIND_GROUP_MESSAGE_BY_ID)
            .bind(message_id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn edit_message_by_id(&self, message_id: i32, content: String) -> Result<GroupMessageRepositoryModel, String> {
        sqlx::query_as::<_, GroupMessageRepositoryModel>(EDIT_MESSAGE_BY_ID_STMT)
            .bind(message_id)
            .bind(content)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| e.to_string())
    }
}
