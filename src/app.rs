use sea_orm::{DatabaseConnection, Database, ConnectOptions};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection
}

impl AppState {
    pub async fn default() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false);
        let db = Database::connect(opt).await.unwrap();
        AppState {
            db 
        }
    }
}
