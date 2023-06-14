use actix::{prelude::*, fut::wrap_future};
// use actix_broker::BrokerIssue;
use actix_web_actors::ws;

use crate::{message::{message::IncomingSessionMessage, server::WsChatServer}, middleware::verify_token};

use super::message::{UserConnects, UserDisconnects, IncomingServerMessage, OutgoingServerMessage};



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
        log::info!("User '{}' is disconnecting", uid);
        let user_disconnect_msg = UserDisconnects {
            user_id: uid,
        };
        WsChatServer::from_registry()
            .send(user_disconnect_msg)
            .into_actor(self)
            .then(|_,_,_| fut::ready(()))
            .wait(ctx);
    }

    pub fn send_message(&self, msg: IncomingSessionMessage, ctx: &mut ws::WebsocketContext<Self>) {
        let Some(sender_uid) = verify_token(self.token.clone()) else {
            return;
        };
        let msg = IncomingServerMessage {
            sender_uid,
            receiver_uid: msg.receiver_uid,
            content: msg.content
        };
        // WsChatServer::from_registry().do_send(msg);
        let fut = async move {
            let res = WsChatServer::from_registry().send(msg).await;
            if let Err(err) = res {
                match err {
                    MailboxError::Closed => log::info!("Mailbox error occurred"),
                    MailboxError::Timeout => log::info!("Mailbox error timeout occurred")
                }
            }
        };
        let fut = wrap_future(fut);
        ctx.spawn(fut);
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
        self.user_disconnects(ctx);
    }
}

impl Handler<OutgoingServerMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: OutgoingServerMessage, ctx: &mut Self::Context) -> Self::Result {
        // let OutgoingServerMessage { content, .. } = msg;
        let Ok(json) = serde_json::to_string(&msg) else {
            log::info!("Issue parsing message");
            return;
        };
        ctx.text(json);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let Ok(msg) = msg else { 
            ctx.stop();
            return;
        };
        match msg {
            ws::Message::Text(text) => {
                let text = text.to_string();
                let Ok(msg) = serde_json::from_str::<IncomingSessionMessage>(&text) else {
                    return;
                };
                // ctx.spawn(fut);
                self.send_message(msg, ctx);
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