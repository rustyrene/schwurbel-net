use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;

use crate::actors::messages::Disconnect;

use super::{
    chat_room::Room,
    lobby::Lobby,
    messages::{ClientMessage, Connect, Message},
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    id: Uuid,
    hb: Instant,
    lobby_addr: Addr<Lobby>,
    room_addr: Option<Addr<Room>>,
}

impl WsConn {
    pub fn new(lobby_addr: Addr<Lobby>) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            hb: Instant::now(),
            lobby_addr,
            room_addr: None,
        }
    }

    pub fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Heartbeat failed. Disconnecting!");
                act.lobby_addr.do_send(Disconnect { user_id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"ping");
        });
    }

    pub fn joinRoom(&mut self, room_addr: Addr<Room>) {
        self.room_addr = Some(room_addr);
    }

    pub fn leaveRoom(&mut self) {
        self.room_addr = None;
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        //connect to the server
        self.lobby_addr
            .send(Connect {
                user_id: self.id.clone(),
                user_addr: addr,
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // Check for potential error
        let msg = match item {
            Ok(msg) => msg,
            _ => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => self.hb = Instant::now(),
            ws::Message::Nop => (),
            ws::Message::Binary(_) => (),
            ws::Message::Continuation(_) => ctx.stop(),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Text(msg) => handle_message(&self, msg.to_string()),
        }
    }
}

impl Handler<Message> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

fn handle_message(ws_conn: &WsConn, msg: String) {
    ws_conn.lobby_addr.do_send(ClientMessage {
        sender_id: ws_conn.id,
        message: msg,
    });
}
