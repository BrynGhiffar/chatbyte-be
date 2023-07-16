use sea_orm::{DatabaseConnection, Database};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection
}

impl AppState {
    pub async fn default() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let db = Database::connect(db_url).await.unwrap();
        AppState {
            db 
        }
    }
}
