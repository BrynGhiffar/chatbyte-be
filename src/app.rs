use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use sea_orm::{DatabaseConnection, Database, ConnectOptions};

use crate::repository::auth_repository::AuthRepository;
use crate::repository::contact_repository::ContactRepository;
use crate::repository::message_repository::MessageRepository;
use crate::repository::user_repository::UserRepository;

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
    pub transmitter: Sender<String>
}

impl AppState {
    pub async fn default() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let env_jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is missing");
        let env_jwt_secret_mins = std::env::var("JWT_EXPIRATION_MINS")
            .expect("JWT_EXPIRATION_MINS is missing")
            .parse::<u64>()
            .expect("JWT_EXPIRATION_MINS cannot be parsed into u64");
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false);
        let empty_profile = Self::read_empty_profile();

        let db = Database::connect(opt).await.unwrap();
        let message_repository = MessageRepository::new(db.clone());
        let contact_repository = ContactRepository::new(db.clone());
        let auth_repository = AuthRepository::new(db.clone());
        let user_repository = UserRepository::new(db.clone());
        let (tx, rx) = channel::<String>();
        thread::spawn(move || { 
            while let Ok(msg) = rx.recv() {
                log::info!("Message: {}", msg);
            }
        });
        AppState {
            db,
            env_jwt_secret,
            env_jwt_secret_mins,
            empty_profile,
            message_repository,
            contact_repository,
            auth_repository,
            user_repository,
            transmitter: tx
        }
    }

    pub fn read_empty_profile() -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        let mut f = File::open("src/assets/empty-profile.jpg").expect("Empty profile image missing");
        f.read_to_end(&mut buffer).expect("Issue when reading file error");
        return buffer;
    }
}
