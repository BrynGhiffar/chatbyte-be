use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChangeUsernameForm {
    pub username: String,
}