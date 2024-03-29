use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{
    messages::{ClientMessage, JoinRoom, Message},
    ws_conn::WsConn,
};

#[derive(Clone)]
pub struct Room {
    pub id: Uuid,
    pub address: Option<Addr<Room>>,
    users: HashMap<Uuid, Addr<WsConn>>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            id: Uuid::new_v4(),
            address: None,
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user_id: Uuid, user_addr: Addr<WsConn>) -> Option<Addr<WsConn>> {
        self.users.insert(user_id, user_addr)
    }

    pub fn _remove_user(&mut self, user_id: Uuid) {
        self.users.remove(&user_id);
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.address = Some(ctx.address());
    }
}

impl Handler<ClientMessage> for Room {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        println!("Send msg");
        for (user_id, user_addr) in self.users.to_owned() {
            if user_id != msg.sender_id {
                user_addr.do_send(Message(msg.message.to_owned()))
            }
        }
    }
}

impl Handler<JoinRoom> for Room {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) {
        // TODO
        self.add_user(msg.user_id, msg.user_addr);
    }
}
