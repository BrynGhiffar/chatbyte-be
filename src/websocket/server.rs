use std::collections::HashMap;
use std::str::FromStr;

use super::session::SessionFactory;
use super::MessageAttachment;
use super::MessageNotificationAttachment;
use super::SessionHandle;
use super::SessionID;
use super::WsRequest;
use super::WsResponse::*;
use super::WsResponse::{self};
use crate::repository::group::GroupRepository;
use crate::repository::message::MessageRepository;
use crate::service::CreateAttachmentModel;
use crate::service::CreateDirectMessageModel;
use crate::service::CreateGroupMessageModel;
use crate::service::DirectMessageModel;
use crate::service::MessageService;
use crate::websocket::message::AppMessage;
use crate::websocket::message::AppRx;
use crate::websocket::message::SessionMessage;
use crate::websocket::message::SessionTx;

pub struct WsServer {
    user_storage: HashMap<SessionID, SessionHandle>,
    app_rx: AppRx,
    message_repository: MessageRepository,
    group_repository: GroupRepository,
    message_service: MessageService,
}

impl WsServer {
    pub fn new(
        message_repository: MessageRepository,
        group_repository: GroupRepository,
        message_service: MessageService,
    ) -> (Self, SessionFactory) {
        let (app_tx, app_rx) = AppMessage::channel();

        let user_storage = HashMap::new();
        let message_repository = message_repository;
        let ws_server = Self {
            user_storage,
            app_rx,
            message_repository,
            group_repository,
            message_service,
        };
        let session_factory = SessionFactory { app_tx };
        (ws_server, session_factory)
    }

    async fn session_up(
        &mut self,
        session_id: SessionID,
        user_id: i32,
        sess_tx: SessionTx,
    ) {
        let prev = self.user_storage.insert(
            session_id,
            SessionHandle {
                user_id,
                sender: sess_tx,
            },
        );
        if let Some(session) = prev {
            let _ = session.sender.send(SessionMessage::CloseConnection);
        }
    }

    fn send_session_message(
        &self,
        user_id: i32,
        message: WsResponse,
    ) {
        self.user_storage
            .iter()
            .filter(|(_, handle)| handle.user_id == user_id)
            .for_each(|(_, handle)| {
                handle
                    .sender
                    .send(SessionMessage::Message(message.to_string()))
                    .unwrap()
            });
    }

    fn send_session_error(
        &self,
        session_id: SessionID,
        message: String,
    ) -> Option<()> {
        let session_handle = self.user_storage.get(&session_id)?;
        session_handle
            .sender
            .send(SessionMessage::Message(
                ErrorNotification { message }.to_string(),
            ))
            .unwrap();
        Some(())
    }

    fn send_new_message_notification(
        &self,
        user_id: i32,
        msg: DirectMessageModel,
    ) {
        let message = MessageNotification {
            id: msg.id,
            sender_uid: msg.sender_id,
            receiver_uid: msg.receiver_id,
            is_user: user_id == msg.sender_id,
            content: msg.content,
            sent_at: msg.sent_at,
            receiver_read: msg.read,
            attachments: msg
                .attachments
                .iter()
                .map(|at| MessageNotificationAttachment {
                    id: at.id,
                    file_type: at.file_type.clone(),
                })
                .collect(),
        };
        self.send_session_message(user_id, message);
    }

    async fn handle_read_message(
        &self,
        session_id: SessionID,
        sender_uid: i32,
    ) -> Option<()> {
        let session_handle = self.user_storage.get(&session_id)?;
        let receiver_uid = session_handle.user_id;
        let message = ReadDirectNotification {
            sender_uid,
            receiver_uid,
        };
        let res = self
            .message_repository
            .update_message_read(receiver_uid, sender_uid)
            .await;
        match res {
            Ok(_) => {}
            Err(e) => return Some(log::info!("{}", e)),
        };
        self.send_session_message(sender_uid, message);
        Some(())
    }

    async fn handle_group_read_message(
        &self,
        session_id: SessionID,
        group_id: i32,
    ) -> Option<()> {
        let SessionHandle { user_id, .. } = self.user_storage.get(&session_id)?;
        let res = self
            .group_repository
            .read_all_message(*user_id, group_id)
            .await;
        match res {
            Ok(succ) if succ => (),
            Ok(_) => log::error!("failed to update group message read"),
            Err(e) => log::error!("{e}"),
        };
        Some(())
    }

    async fn handle_user_send_message(
        &self,
        session_id: SessionID,
        receiver_uid: i32,
        msg: String,
        attachments: Vec<MessageAttachment>,
    ) -> Option<()> {
        let handle = self.user_storage.get(&session_id)?;
        let sender_uid = handle.user_id;
        let mut create_attachments = Vec::<CreateAttachmentModel>::new();
        for at in attachments {
            let bytes = match at.content_as_bytes() {
                Ok(b) => b,
                Err(e) => {
                    log::error!("{e}");
                    continue;
                }
            };
            create_attachments.push(CreateAttachmentModel {
                name: at.name.clone(),
                attachment: bytes,
            });
        }
        let res = self
            .message_service
            .create_direct_message(CreateDirectMessageModel {
                receiver_id: receiver_uid,
                sender_id: sender_uid,
                content: msg,
                attachment: create_attachments,
            })
            .await;
        let msg = match res {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("{e}");
                self.send_session_error(session_id, e.to_string());
                return Some(());
            }
        };
        self.send_new_message_notification(sender_uid, msg.clone());
        self.send_new_message_notification(receiver_uid, msg);
        Some(())
    }

    async fn handle_send_group_message(
        &self,
        session_id: SessionID,
        group_id: i32,
        message: String,
        attachments: Vec<MessageAttachment>,
    ) -> Option<()> {
        let SessionHandle {
            user_id: sender_uid,
            ..
        } = self.user_storage.get(&session_id)?;
        let result = self.group_repository.find_group_members(group_id).await;
        let member_ids = match result {
            Ok(ids) => ids,
            Err(e) => return Some(log::error!("{e}")),
        };
        if !member_ids.contains(&sender_uid) {
            log::error!("User with id '{sender_uid}' is not part of group with id '{group_id}'");
            return Some(());
        }
        let mut create_attachments = Vec::<CreateAttachmentModel>::new();
        for at in attachments {
            let bytes = match at.content_as_bytes() {
                Ok(b) => b,
                Err(e) => {
                    log::error!("{e}");
                    continue;
                }
            };
            create_attachments.push(CreateAttachmentModel {
                name: at.name.clone(),
                attachment: bytes,
            });
        }
        let res = self
            .message_service
            .create_group_message(CreateGroupMessageModel {
                group_id,
                sender_id: *sender_uid,
                content: message,
                attachment: create_attachments,
            })
            .await;
        let message = match res {
            Ok(m) => WsResponse::from_group_message(m),
            Err(e) => return Some(log::error!("{e}")),
        };
        for mid in member_ids.iter() {
            self.send_session_message(*mid, message.clone());
        }
        Some(())
    }

    async fn handle_delete_direct_message(
        &self,
        session_id: SessionID,
        message_id: i32,
    ) -> Option<()> {
        let SessionHandle {
            user_id: sender_id, ..
        } = self.user_storage.get(&session_id)?;
        let result = self.message_repository.find_message_by_id(message_id).await;
        let message = match result {
            Ok(Some(message)) => message,
            Ok(None) => return Some(()),
            Err(e) => return Some(log::info!("{e}")),
        };
        if message.sender_id != *sender_id {
            return Some(());
        }
        let result = self.message_repository.delete_message(message_id).await;
        let success = match result {
            Ok(succ) => succ,
            Err(e) => return Some(log::error!("{e}")),
        };
        if !success {
            return Some(());
        }
        let sender_message = WsResponse::DeleteMessageNotification {
            contact_id: message.receiver_id,
            message_id,
        };
        let receiver_message = WsResponse::DeleteMessageNotification {
            contact_id: message.sender_id,
            message_id,
        };
        self.send_session_message(message.sender_id, sender_message);
        self.send_session_message(message.receiver_id, receiver_message);
        Some(())
    }

    async fn handle_delete_group_message(
        &self,
        session_id: SessionID,
        message_id: i32,
    ) -> Option<()> {
        use WsResponse::*;
        let session_handle = self.user_storage.get(&session_id)?;
        let result = self.group_repository.find_message_by_id(message_id).await;
        let message = match result {
            Ok(Some(mess)) => mess,
            Ok(None) => return Some(()),
            Err(e) => return Some(log::info!("{}", e)),
        };
        if message.sender_id != session_handle.user_id {
            return Some(());
        }
        let result = self
            .group_repository
            .set_message_to_delete(message_id)
            .await;
        let success = match result {
            Ok(succ) => succ,
            Err(e) => return Some(log::info!("{e}")),
        };
        if !success {
            return Some(());
        }
        let result = self
            .group_repository
            .find_group_members(message.group_id)
            .await;
        let members = match result {
            Ok(mems) => mems,
            Err(e) => return Some(log::info!("{e}")),
        };
        for user_id in members {
            self.send_session_message(
                user_id,
                DeleteGroupMessageNotification {
                    group_id: message.group_id,
                    message_id: message.id,
                },
            );
        }
        Some(())
    }

    async fn handle_edit_direct_message(
        &self,
        session_id: SessionID,
        message_id: i32,
        edited_content: String,
    ) -> Option<()> {
        let session_handle = self.user_storage.get(&session_id)?;
        let result = self.message_repository.find_message_by_id(message_id).await;
        let message = match result {
            Ok(Some(mess)) => mess,
            Ok(None) => return Some(()),
            Err(e) => return Some(log::error!("{e}")),
        };
        if message.sender_id != session_handle.user_id {
            return Some(());
        }
        let result = self
            .message_repository
            .edit_message_by_id(message_id, edited_content.clone())
            .await;
        let message = match result {
            Ok(mess) => mess,
            Err(e) => return Some(log::error!("{e}")),
        };

        self.send_session_message(
            message.sender_id,
            UpdateDirectMessageNotification {
                contact_id: message.receiver_id,
                message_id,
                content: edited_content.clone(),
            },
        );

        self.send_session_message(
            message.receiver_id,
            UpdateDirectMessageNotification {
                contact_id: message.sender_id,
                message_id,
                content: edited_content.clone(),
            },
        );
        Some(())
    }

    async fn handle_edit_group_message(
        &self,
        session_id: SessionID,
        message_id: i32,
        edited_content: String,
    ) -> Option<()> {
        let session_handle = self.user_storage.get(&session_id)?;
        let result = self.group_repository.find_message_by_id(message_id).await;
        let message = match result {
            Ok(Some(mess)) => mess,
            Ok(None) => return Some(()),
            Err(e) => return Some(log::error!("{e}")),
        };
        if message.sender_id != session_handle.user_id {
            return Some(());
        }
        let result = self
            .group_repository
            .edit_message_by_id(message_id, edited_content.clone())
            .await;
        let _ = match result {
            Ok(mess) => mess,
            Err(e) => return Some(log::error!("{e}")),
        };
        let result = self
            .group_repository
            .find_group_members(message.group_id)
            .await;
        let members = match result {
            Ok(mem) => mem,
            Err(e) => return Some(log::error!("{e}")),
        };
        for member_id in members {
            self.send_session_message(
                member_id,
                UpdateGroupMessageNotification {
                    group_id: message.group_id,
                    message_id,
                    content: edited_content.clone(),
                },
            );
        }
        Some(())
    }

    async fn session_message(
        &mut self,
        session_id: SessionID,
        msg: String,
    ) {
        let message = match WsRequest::from_str(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("parsing error: {e}");
                return;
            }
        };
        match message {
            WsRequest::SendMessage {
                receiver_uid,
                message,
                attachments,
            } => {
                self.handle_user_send_message(session_id, receiver_uid, message, attachments)
                    .await;
            }
            WsRequest::SendGroupMessage {
                group_id,
                message,
                attachments,
            } => {
                self.handle_send_group_message(session_id, group_id, message, attachments)
                    .await;
            }
            WsRequest::ReadDirectMessage { receiver_uid } => {
                self.handle_read_message(session_id, receiver_uid).await;
            }
            WsRequest::ReadGroupMessage { group_id } => {
                self.handle_group_read_message(session_id, group_id).await;
            }
            WsRequest::DeleteDirectMessage { message_id } => {
                self.handle_delete_direct_message(session_id, message_id)
                    .await;
            }
            WsRequest::DeleteGroupMessage { message_id } => {
                self.handle_delete_group_message(session_id, message_id)
                    .await;
            }
            WsRequest::EditDirectMessage {
                message_id,
                edited_content,
            } => {
                self.handle_edit_direct_message(session_id, message_id, edited_content)
                    .await;
            }
            WsRequest::EditGroupMessage {
                message_id,
                edited_content,
            } => {
                self.handle_edit_group_message(session_id, message_id, edited_content)
                    .await;
            }
        };
    }

    async fn session_down(
        &mut self,
        session_id: SessionID,
    ) {
        let sess = self.user_storage.remove(&session_id);
        if let Some(sess) = sess {
            let _ = sess.sender.send(SessionMessage::CloseConnection);
        }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        while let Some(msg) = self.app_rx.recv().await {
            match msg {
                AppMessage::Connect {
                    session_id,
                    user_id,
                    sess_tx,
                } => self.session_up(session_id, user_id, sess_tx).await,
                AppMessage::Message {
                    session_id,
                    message,
                } => self.session_message(session_id, message).await,
                AppMessage::Disconnect { session_id } => self.session_down(session_id).await,
            }
        }
        Ok(())
    }
}
