use sqlx::postgres::PgPoolOptions;
use std::fs::File;
use std::io::prelude::*;

use crate::repository::AttachmentRepository;
use crate::repository::AuthRepository;
use crate::repository::ContactRepository;
use crate::repository::GroupRepository;
use crate::repository::MessageRepository;
use crate::repository::SessionRepository;
use crate::repository::UserRepository;
use crate::service::AttachmentService;
use crate::service::AuthService;
use crate::service::ContactService;
use crate::service::GroupService;
use crate::service::MessageService;
use crate::service::UserService;
use crate::websocket::SessionFactory;
use crate::websocket::WsServer;

#[derive(Clone)]
pub struct AppState {
    pub env_jwt_secret: String,
    pub env_jwt_secret_mins: u64,
    pub empty_profile: Vec<u8>,
    pub message_repository: MessageRepository,
    pub contact_repository: ContactRepository,
    pub auth_repository: AuthRepository,
    pub user_repository: UserRepository,
    pub session_repository: SessionRepository,
    pub session_factory: SessionFactory,
    pub group_repository: GroupRepository,
    pub attachment_repository: AttachmentRepository,
    pub auth_service: AuthService,
    pub contact_service: ContactService,
    pub message_service: MessageService,
    pub group_service: GroupService,
    pub user_service: UserService,
    pub attachment_service: AttachmentService,
}

impl AppState {
    pub async fn default() -> (Self, WsServer) {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let env_jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is missing");
        let env_jwt_secret_mins = std::env::var("JWT_EXPIRATION_MINS")
            .expect("JWT_EXPIRATION_MINS is missing")
            .parse::<u64>()
            .expect("JWT_EXPIRATION_MINS cannot be parsed into u64");
        let sqlx_conn = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        let empty_profile = Self::read_empty_profile();

        let message_repository = MessageRepository::new(sqlx_conn.clone());
        let contact_repository = ContactRepository::new(sqlx_conn.clone());
        let auth_repository = AuthRepository::new(sqlx_conn.clone());
        let user_repository = UserRepository::new(sqlx_conn.clone());
        let session_repository = SessionRepository::new(sqlx_conn.clone());
        let group_repository = GroupRepository::new(sqlx_conn.clone());
        let attachment_repository = AttachmentRepository::new(sqlx_conn.clone());
        let message_service = MessageService::new(sqlx_conn.clone());
        let auth_service = AuthService::new(
            auth_repository.clone(),
            env_jwt_secret.clone(),
            env_jwt_secret_mins,
        );
        let contact_service = ContactService::new(
            contact_repository.clone(),
            message_repository.clone(),
            group_repository.clone(),
        );
        let (ws_server, session_factory) = WsServer::new(
            message_repository.clone(),
            group_repository.clone(),
            message_service.clone(),
        );
        let group_service = GroupService::new(
            sqlx_conn.clone(),
            group_repository.clone(),
            empty_profile.clone(),
        );
        let user_service = UserService::new(user_repository.clone(), auth_repository.clone());
        let attachment_service = AttachmentService::new(attachment_repository.clone());
        let app_state = AppState {
            env_jwt_secret,
            env_jwt_secret_mins,
            empty_profile,
            message_repository,
            contact_repository,
            auth_repository,
            user_repository,
            session_factory,
            session_repository,
            group_repository,
            attachment_repository,
            auth_service,
            contact_service,
            message_service,
            group_service,
            user_service,
            attachment_service,
        };
        (app_state, ws_server)
    }

    pub fn read_empty_profile() -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        let mut f =
            File::open("src/assets/empty-profile.jpg").expect("Empty profile image missing");
        f.read_to_end(&mut buffer)
            .expect("Issue when reading file error");
        return buffer;
    }
}
