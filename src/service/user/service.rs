use anyhow::anyhow;

use crate::repository::AuthRepository;
use crate::repository::UserRepository;

use super::SuccessfullyUpdateUser;
use super::UserDetail;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
    auth_repository: AuthRepository
}

impl UserService {
    pub fn new(
        user_repository: UserRepository,
        auth_repository: AuthRepository
    ) -> Self {
        Self { 
            user_repository,
            auth_repository
        }
    }

    pub async fn find_user_details(
        &self,
        user_id: i32
    ) -> Result<UserDetail, anyhow::Error> {
        let res = self.auth_repository
            .find_user_by_id(user_id)
            .await
            .map_err(|e| anyhow!(e))?
            .ok_or(anyhow!("User not found"))?;
        Ok(UserDetail {
            user_id: res.id,
            username: res.username
        })

    }

    pub async fn find_user_avatar(
        &self,
        user_id: i32,
    ) -> Option<Vec<u8>> {
        let res = self
            .user_repository
            .get_avatar(user_id)
            .await;
        res.ok().flatten().map(|s| s.avatar_image)
    }

    pub async fn update_username(
        &self,
        user_id: i32,
        new_username: String,
    ) -> Result<SuccessfullyUpdateUser, anyhow::Error> {
        if new_username.len() < 5 {
            return Err(anyhow!("Username length must be at least 5 characters long"))
        }
        let success = self.auth_repository
            .update_username(user_id, new_username)
            .await
            .map_err(|e| anyhow!(e))?;
        if !success {
            return Err(anyhow!("Failed updating username"));
        }
        Ok(SuccessfullyUpdateUser)
    }

    pub async fn update_user_avatar(
        &self,
        user_id: i32,
        profile_picture: Vec<u8>
    ) -> Result<SuccessfullyUpdateUser, anyhow::Error> {
        let success = self.user_repository
            .upsert_user_profile(user_id, profile_picture)
            .await
            .map_err(|e| anyhow!(e))?;
        if !success {
            return Err(anyhow!("Failed updating user profile"));
        }
        Ok(SuccessfullyUpdateUser)
    }
}