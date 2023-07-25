use std::fs::File;
use std::io::prelude::*;

use sea_orm::{DatabaseConnection, Database, ConnectOptions};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub env_jwt_secret: String,
    pub env_jwt_secret_mins: u64,
    pub empty_profile: Vec<u8>
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
        AppState {
            db,
            env_jwt_secret,
            env_jwt_secret_mins,
            empty_profile
        }
    }

    pub fn read_empty_profile() -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        let mut f = File::open("src/assets/empty-profile.jpg").expect("Empty profile image missing");
        f.read_to_end(&mut buffer).expect("Issue when reading file error");
        return buffer;
    }
}
