use serde::Serialize;

#[derive(Serialize)]
pub struct UserDetail {
    pub user_id: i32,
    pub username: String
}

pub struct SuccessfullyUpdateUser;

impl Serialize for SuccessfullyUpdateUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str("Successfully updated user")
    }
}