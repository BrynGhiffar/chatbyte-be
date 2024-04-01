use anyhow::bail;
use sqlx::Acquire;
use sqlx::PgPool;

use crate::repository::GroupImage;
use crate::repository::GroupRepository;

use super::CreateGroupForm;
use super::GroupModel;

#[derive(Clone)]
pub struct GroupService {
    conn: PgPool,
    group_repository: GroupRepository,
    empty_profile: Vec<u8>,
}

impl GroupService {
    pub fn new(
        conn: PgPool,
        group_repository: GroupRepository,
        empty_profile: Vec<u8>,
    ) -> Self {
        Self {
            conn,
            group_repository,
            empty_profile,
        }
    }

    pub async fn create_group(
        &self,
        user_id: i32,
        CreateGroupForm {
            group_name,
            mut members,
            profile_picture,
        }: CreateGroupForm,
    ) -> Result<GroupModel, anyhow::Error> {
        log::info!("Creating group");
        let mut tx = self.conn.begin().await?;

        let conn = tx.acquire().await?;
        let res = GroupRepository::create_group_with_executor(conn, group_name).await;
        let group = match res {
            Ok(g) => g,
            Err(e) => bail!(e),
        };
        if !members.contains(&user_id) {
            members.push(user_id);
        }
        log::info!("Adding group members");
        let group_id = group.id;
        for mem in members {
            let conn = tx.acquire().await?;
            let res = GroupRepository::add_user_to_group_with_executor(conn, mem, group_id).await;
            match res {
                Ok(succ) if succ => continue,
                Ok(_) => bail!("Error adding user to group"),
                Err(e) => bail!(e),
            };
        }
        log::info!("adding profile pictures");
        let conn = tx.acquire().await?;
        let res = GroupRepository::set_profile_image_for_group_with_executor(
            conn,
            group.id,
            profile_picture.unwrap_or(self.empty_profile.clone()),
        )
        .await;
        match res {
            Ok(s) if !s => bail!("Failed adding profile picture"),
            Err(e) => bail!(e),
            Ok(_) => {}
        };
        tx.commit().await?;
        log::info!("Finished creating group");
        Ok(GroupModel {
            id: group.id,
            name: group.name.clone(),
        })
    }

    pub async fn find_group_profile_image(
        &self,
        group_id: i32,
    ) -> Vec<u8> {
        let res = self
            .group_repository
            .get_profile_image_for_group(group_id)
            .await;
        match res {
            Ok(Some(GroupImage(img))) => img,
            _ => self.empty_profile.clone(),
        }
    }
}
