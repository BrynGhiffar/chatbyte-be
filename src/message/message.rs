use actix::prelude::*;
use sea_orm::DatabaseConnection;

use super::session::WsChatSession;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct UserConnects {
    pub user_id: i32,
    pub addr: Addr<WsChatSession>
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct UserDisconnects {
    pub user_id: i32
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub token: String,
    pub content: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ConnectDatabase(pub DatabaseConnection);