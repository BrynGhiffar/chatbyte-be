use std::collections::HashMap;
use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use sea_orm::{Database, DatabaseConnection};

use crate::message::message::{ChatMessage, LeaveRoom, JoinRoom, ListRooms, SendMessage};

use super::message::ConnectDatabase;

type Client = Recipient<ChatMessage>;
type Room = HashMap<usize, Client>;

// #[derive(Default)]
pub struct WsChatServer {
    rooms: HashMap<String, Room>,
    db: Option<DatabaseConnection>
}

impl Default for WsChatServer {

    fn default() -> Self {
        return WsChatServer { 
            rooms: HashMap::new(), 
            db: None
        };
    }
}

impl WsChatServer {

    fn take_room(&mut self, room_name: &str) -> Option<Room> {
        let room = self.rooms.get_mut(room_name)?;
        let room = std::mem::take(room);
        Some(room)
    }

    fn add_client_to_room(&mut self, room_name: &str, id: Option<usize>, client: Client) -> usize {
        let mut id = id.unwrap_or_else(rand::random::<usize>);

        if let Some(room) = self.rooms.get_mut(room_name) {
            loop {
                if room.contains_key(&id) {
                    id = rand::random::<usize>();
                } else {
                    break;
                }
            }
            room.insert(id, client);
            return id;
        }

        let mut room: Room = HashMap::new();
        room.insert(id, client);
        self.rooms.insert(room_name.to_owned(), room);
        id
    }

    fn send_chat_message(&mut self, room_name: &str, msg: &str, _src: usize) -> Option<()> {
        let mut room = self.take_room(room_name)?;

        for (id, client) in room.drain() {
            if client.try_send(ChatMessage(msg.to_owned())).is_ok() {
                self.add_client_to_room(room_name, Some(id), client);
            }
        }

        Some(())
    }
}

impl Actor for WsChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveRoom>(ctx);
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

impl Handler<JoinRoom> for WsChatServer {

    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client_name, client) = msg;
        let id = self.add_client_to_room(&room_name, None, client);
        let join_msg = format!(
            "{} join {room_name}",
            client_name.unwrap_or_else(|| "anon".to_string())
        );
        self.send_chat_message(&room_name, &join_msg, id);
        MessageResult(id)
    }
}

impl Handler<LeaveRoom> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.0) {
            room.remove(&msg.1);
        }
    }
}

impl Handler<ListRooms> for WsChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _msg: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

impl Handler<SendMessage> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) -> Self::Result {
        let SendMessage(room_name, id, msg) = msg;
        self.send_chat_message(&room_name, &msg, id);
    }
}


impl SystemService for WsChatServer { }
impl Supervised for WsChatServer {}