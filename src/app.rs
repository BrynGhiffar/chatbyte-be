use std::fs::File;
use std::io::prelude::*;
use sqlx::postgres::PgPoolOptions;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::repository::auth_repository::AuthRepository;
use crate::repository::contact_repository::ContactRepository;
use crate::repository::group_repository::GroupRepository;
use crate::repository::message_repository::MessageRepository;
use crate::repository::session_repository::SessionRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::session::SessionFactory;
use crate::service::ws_server::WsServer;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
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
            .connect(&db_url).await.unwrap();
        let mut opt = ConnectOptions::new(db_url.clone());
        opt.sqlx_logging(false);
        let empty_profile = Self::read_empty_profile();

        let db = Database::connect(opt).await.unwrap();
        let message_repository = MessageRepository::new(db.clone());
        let contact_repository = ContactRepository::new(db.clone());
        let auth_repository = AuthRepository::new(db.clone());
        let user_repository = UserRepository::new(db.clone());
        let session_repository = SessionRepository::new(db.clone());
        let group_repository = GroupRepository::new(sqlx_conn.clone());
        let (ws_server, session_factory) = WsServer::new(
            message_repository.clone(),
            group_repository.clone()
        );
        let app_state = AppState {
            db,
            env_jwt_secret,
            env_jwt_secret_mins,
            empty_profile,
            message_repository,
            contact_repository,
            auth_repository,
            user_repository,
            session_factory,
            session_repository,
            group_repository
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
