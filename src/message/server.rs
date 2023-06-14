use actix::prelude::*;
// use actix_broker::BrokerSubscribe;
use sea_orm::{Database, DatabaseConnection};
use std::collections::HashMap;

use super::{
    message::{ConnectDatabase, UserConnects, UserDisconnects, IncomingServerMessage, OutgoingServerMessage},
    session::WsChatSession,
};

// #[derive(Default)]
pub struct WsChatServer {
    clients: HashMap<i32, Addr<WsChatSession>>,
    db: Option<DatabaseConnection>,
}

impl Default for WsChatServer {
    fn default() -> Self {
        return WsChatServer {
            clients: HashMap::new(),
            db: None,
        };
    }
}

impl WsChatServer {
    fn send_message(
        &mut self, 
        recv: i32, 
        msg: OutgoingServerMessage, 
        ctx: &mut <Self as Actor>::Context
    ) {
        if let Some(receiver_addr) = self.clients.get(&recv) { 
            let receiver_addr = receiver_addr.clone();
            let msg = msg.clone();
            let fut = async move {
                let res = receiver_addr.send(msg).await;
                if let Err(err) = res {
                    match err {
                        MailboxError::Closed => log::info!("Mailbox error closed occurred"),
                        MailboxError::Timeout => log::info!("Mailbox error timeout occurred")
                    }
                };
            };
            let fut = actix::fut::wrap_future::<_, Self>(fut);
            ctx.spawn(fut);
        }
    }
}

impl Actor for WsChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(5);
        let addr = ctx.address();
        let fut = async move {
            let db_url = std::env::var("DATABASE_URL").expect("DATABSE_URL is missing");
            let db = Database::connect(db_url).await.unwrap();
            addr.send(ConnectDatabase(db)).await.unwrap();
        };
        let fut = actix::fut::wrap_future::<_, Self>(fut);
        ctx.spawn(fut);
    }
}

impl Handler<ConnectDatabase> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: ConnectDatabase, _ctx: &mut Self::Context) -> Self::Result {
        let ConnectDatabase(conn) = msg;
        if self.db.is_none() {
            self.db = Some(conn);
        }
    }
}

impl Handler<UserConnects> for WsChatServer {
    type Result = ();
    fn handle(&mut self, msg: UserConnects, _ctx: &mut Self::Context) -> Self::Result {
        let UserConnects { user_id, addr } = msg;
        self.clients.insert(user_id, addr);
    }
}

impl Handler<UserDisconnects> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: UserDisconnects, _ctx: &mut Self::Context) -> Self::Result {
        let UserDisconnects { user_id } = msg;
        self.clients.remove(&user_id);
    }
}

impl Handler<IncomingServerMessage> for WsChatServer {
    type Result = ();
    fn handle(&mut self, msg: IncomingServerMessage, ctx: &mut Self::Context) -> Self::Result {
        let IncomingServerMessage { 
            sender_uid, 
            receiver_uid, 
            content 
        } = msg;
        let msg = OutgoingServerMessage {
            sender_uid,
            receiver_uid,
            content
        };

        self.send_message(receiver_uid, msg.clone(), ctx);
        self.send_message(sender_uid, msg.clone(), ctx);
    }
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
