use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_web_actors::ws;

use crate::{message::{message::{ChatMessage, SendMessage}, server::WsChatServer}, middleware::verify_token};

use super::message::{UserConnects, UserDisconnects};



// #[derive(Default)]
pub struct WsChatSession {
    token: String,
    // id: usize,
    // room: String,
    // name: Option<String>
}

impl WsChatSession {

    pub fn new(token: String) -> Self {
        Self { token }
    }


    pub fn user_connects(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        let res = verify_token(self.token.clone());
        let Some(uid) = res else { ctx.close(None); return; };
        log::info!("User '{}' is online", uid);
        let user_connect_msg = UserConnects {
            user_id: uid,
            addr
        };
        WsChatServer::from_registry()
            .send(user_connect_msg)
            .into_actor(self)
            .then(|_,_,_| fut::ready(()))
            .wait(ctx);
    }

    pub fn user_disconnects(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let res = verify_token(self.token.clone());
        let Some(uid) = res else { ctx.close(None); return; };
        let user_disconnect_msg = UserDisconnects {
            user_id: uid,
        };
        WsChatServer::from_registry()
            .send(user_disconnect_msg)
            .into_actor(self)
            .then(|_,_,_| fut::ready(()))
            .wait(ctx);
    }

    // pub fn send_message(&self, msg: &str) {
    //     let content = format!(
    //         "{}: {msg}",
    //         "anon"
    //     );

    //     let msg = SendMessage {
    //         token: self.token,
    //         content
    //     };

    //     self.issue_system_async(msg);
    // }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.user_connects(ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::info!("User disconnects");
        self.user_disconnects(ctx);
    }
}

impl Handler<ChatMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let Ok(msg) = msg else { 
            ctx.stop();
            return;
        };
        let Some(uid) = verify_token(self.token.clone()) else { ctx.close(None); return; };

        match msg {
            ws::Message::Text(text) => {
                log::info!("user {}: {}", uid, text);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
                return;
            },
            _ => {}
        };
    }
}