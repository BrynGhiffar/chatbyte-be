use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}