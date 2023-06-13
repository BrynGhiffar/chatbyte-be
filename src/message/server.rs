use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use sea_orm::{Database, DatabaseConnection};
use std::collections::HashMap;

use crate::message::message::{ChatMessage, SendMessage};

use super::{
    message::{ConnectDatabase, UserConnects, UserDisconnects},
    session::WsChatSession,
};

type Client = Recipient<ChatMessage>;
type Room = HashMap<usize, Client>;

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
    // fn send_chat_message(&mut self, room_name: &str, msg: &str, _src: usize) -> Option<()> {
    //     let mut room = self.take_room(room_name)?;

    //     for (id, client) in room.drain() {
    //         if client.try_send(ChatMessage(msg.to_owned())).is_ok() {
    //             self.add_client_to_room(room_name, Some(id), client);
    //         }
    //     }

    //     Some(())
    // }
}

impl Actor for WsChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<SendMessage>(ctx);

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

impl Handler<SendMessage> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) -> Self::Result {
        // let SendMessage(room_name, id, msg) = msg;
        // self.send_chat_message(&room_name, &msg, id);
    }
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
