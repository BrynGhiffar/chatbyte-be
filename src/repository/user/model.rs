#[derive(sqlx::FromRow)]
pub struct UserAvatar {
    pub user_id: i32,
    pub avatar_image: Vec<u8>,
}
