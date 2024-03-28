use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{
    chat_room::Room,
    messages::{Connect, Disconnect},
    ws_conn::ws_conn,
};

pub struct Lobby {
    pub sessions: HashMap<Uuid, Addr<ws_conn>>,
    pub chat_rooms: HashMap<Uuid, Addr<Room>>,
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
