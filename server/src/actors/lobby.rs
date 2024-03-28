use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{
    chat_room::Room,
    messages::{ClientMessage, Connect, Disconnect, Message},
    ws_conn::WsConn,
};

pub struct Lobby {
    pub sessions: HashMap<Uuid, Addr<WsConn>>,
    pub chat_rooms: HashMap<Uuid, Addr<Room>>,
}

impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            chat_rooms: HashMap::new(),
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        self.sessions.remove(&msg.user_id);
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.user_id, msg.user_addr);
    }
}

impl Handler<ClientMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        for (user_id, user_addr) in self.sessions.clone() {
            if user_id != msg.sender_id {
                user_addr.do_send(Message(msg.message.clone()))
            }
        }
    }
}
