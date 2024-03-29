use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::ws_conn::WsConn;

pub struct Room {
    pub id: Uuid,
    users: HashMap<Uuid, Addr<WsConn>>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            id: Uuid::new_v4(),
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user_id: Uuid, user_addr: Addr<WsConn>) -> Option<Addr<WsConn>> {
        self.users.insert(user_id, user_addr)
    }

    pub fn remove_user(&mut self, user_id: Uuid) {
        self.users.remove(&user_id);
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}
