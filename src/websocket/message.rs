use tokio::sync::mpsc;

use super::SessionID;

pub type AppTx = mpsc::UnboundedSender<AppMessage>;
pub type AppRx = mpsc::UnboundedReceiver<AppMessage>;
pub type SessionTx = mpsc::UnboundedSender<SessionMessage>;
pub type SessionRx = mpsc::UnboundedReceiver<SessionMessage>;

pub enum AppMessage {
    Connect {
        session_id: SessionID,
        user_id: i32,
        sess_tx: mpsc::UnboundedSender<SessionMessage>,
    },
    Message {
        session_id: SessionID,
        message: String,
    },
    Disconnect {
        session_id: SessionID,
    },
}

impl AppMessage {
    pub fn channel() -> (AppTx, AppRx) {
        mpsc::unbounded_channel::<AppMessage>()
    }
}

pub enum SessionMessage {
    CloseConnection,
    Message(String),
}

impl SessionMessage {
    pub fn channel() -> (SessionTx, SessionRx) {
        mpsc::unbounded_channel::<SessionMessage>()
    }
}
