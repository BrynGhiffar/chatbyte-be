use actix::{fut::wrap_future, prelude::*};
use actix_web_actors::ws::{self, CloseReason};

use crate::{
    message::{message::IncomingSessionMessage, server::WsChatServer},
    middleware::verify_token,
};

use super::message::{IncomingServerMessage, OutgoingServerMessage, UserConnects, UserDisconnects};

pub struct WsChatSession {
    token: String,
}

impl WsChatSession {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn user_connects(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        let res = verify_token(self.token.clone());
        let Ok(uid) = res else { ctx.close(None); return; };
        log::info!("User '{}' is online", uid);
        let user_connect_msg = UserConnects { user_id: uid, addr };
        WsChatServer::from_registry()
            .send(user_connect_msg)
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);
    }

    pub fn user_disconnects(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let res = verify_token(self.token.clone());
        let Ok(uid) = res else { ctx.close(None); return; };
        log::info!("User '{}' is disconnecting", uid);
        let user_disconnect_msg = UserDisconnects { user_id: uid };
        WsChatServer::from_registry()
            .send(user_disconnect_msg)
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);
    }

    pub fn send_message(&self, msg: IncomingSessionMessage, ctx: &mut ws::WebsocketContext<Self>) {
        let Ok(sender_uid) = verify_token(self.token.clone()) else {
            return;
        };
        let msg = IncomingServerMessage {
            sender_uid,
            receiver_uid: msg.receiver_uid,
            content: msg.content,
        };
        // WsChatServer::from_registry().do_send(msg);
        let fut = async move {
            let res = WsChatServer::from_registry().send(msg).await;
            if let Err(err) = res {
                match err {
                    MailboxError::Closed => log::info!("Mailbox error occurred"),
                    MailboxError::Timeout => log::info!("Mailbox error timeout occurred"),
                }
            }
        };
        let fut = wrap_future(fut);
        ctx.spawn(fut);
    }

    fn on_session_receive_message(&self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        let Ok(msg) = serde_json::from_str::<IncomingSessionMessage>(&text) else {
            return;
        };
        self.send_message(msg, ctx);
    }

    fn on_session_close(&self, reason: Option<CloseReason>, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.close(reason);
        ctx.stop();
    }
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
            ws::Message::Text(text) => self.on_session_receive_message(text.to_string(), ctx),
            ws::Message::Close(reason) => self.on_session_close(reason, ctx),
            _ => {}
        };
    }
}
