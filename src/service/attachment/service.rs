use anyhow::bail;

use crate::repository::AttachmentFileType;
use crate::repository::AttachmentRepository;
use crate::repository::AttachmentRepositoryModel;

#[derive(Clone)]
pub struct AttachmentService {
    attachment_repository: AttachmentRepository,
}

impl AttachmentService {
    pub fn new(attachment_repository: AttachmentRepository) -> Self {
        Self {
            attachment_repository,
        }
    }

    pub async fn find_attachment_image_by_id(
        &self,
        attachment_id: i32,
    ) -> Result<(Vec<u8>, AttachmentFileType), anyhow::Error> {
        let res = self
            .attachment_repository
            .find_attachment_by_id(attachment_id)
            .await;
        let AttachmentRepositoryModel {
            attachment,
            file_type,
            ..
        } = match res {
            Ok(Some(att)) => att,
            Ok(None) => {
                bail!("Attachment not found")
            }
            Err(e) => bail!(e),
        };
        return Ok((attachment, file_type));
    }
}
