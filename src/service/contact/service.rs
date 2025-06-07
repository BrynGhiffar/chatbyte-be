use anyhow::bail;

use crate::repository::ContactRepository;
use crate::repository::GroupRepository;
use crate::repository::MessageRepository;

use super::DirectContact;
use super::DirectConversation;
use super::GroupContact;
use super::GroupConversation;

#[derive(Clone)]
pub struct ContactService {
    contact_repository: ContactRepository,
    message_repository: MessageRepository,
    group_repository: GroupRepository,
}

impl ContactService {
    pub fn new(
        contact_repository: ContactRepository,
        message_repository: MessageRepository,
        group_repository: GroupRepository,
    ) -> Self {
        Self {
            contact_repository,
            message_repository,
            group_repository,
        }
    }

    pub async fn find_direct_contacts_for_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<DirectContact>, anyhow::Error> {
        let res = self.contact_repository.get_contacts(user_id).await;
        match res {
            Ok(vec) => Ok(vec.iter().map(DirectContact::from).collect()),
            Err(e) => bail!(e),
        }
    }

    pub async fn find_direct_conversations_for_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<DirectConversation>, anyhow::Error> {
        let res = self.message_repository.get_recent_messages(user_id).await;
        let conv = match res {
            Ok(conv) => conv,
            Err(e) => bail!(e),
        };
        Ok(conv.iter().map(DirectConversation::from).collect())
    }

    pub async fn find_group_contacts_for_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<GroupContact>, anyhow::Error> {
        let res = self.group_repository.find_groups_for_user(user_id).await;

        let groups = match res {
            Ok(g) => g,
            Err(e) => bail!(e),
        };

        Ok(groups.iter().map(GroupContact::from).collect())
    }

    pub async fn find_group_conversations_for_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<GroupConversation>, anyhow::Error> {
        let res = self.group_repository.find_user_group_recent(user_id).await;

        let conversations = match res {
            Ok(c) => c,
            Err(e) => bail!(e),
        };

        Ok(conversations.iter().map(GroupConversation::from).collect())
    }

    pub async fn add_user_contact(
        &self,
        user_id: i32,
        contact_id: i32
    ) -> Result<bool, anyhow::Error> {
        todo!()
    }
}
