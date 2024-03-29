use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{
    chat_room::Room,
    messages::{ClientMessage, Connect, CreateRoom, Disconnect, JoinRoom, Message},
    ws_conn::WsConn,
};

pub struct Lobby {
    pub sessions: HashMap<Uuid, Addr<WsConn>>,
    pub chat_rooms: HashMap<Uuid, Room>,
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

impl Handler<CreateRoom> for Lobby {
    type Result = Addr<Room>;

    fn handle(&mut self, msg: CreateRoom, _ctx: &mut Self::Context) -> Addr<Room> {
        println!("Creating!");
        let creater_addr = self
            .sessions
            .get(&msg.creater_id)
            .expect("Session not found");
        let mut room = Room::new();
        let room_id = room.id.clone();
        println!("{}", room_id);
        room.add_user(msg.creater_id, creater_addr.clone());
        let room_addr = room.clone().start();
        self.chat_rooms.insert(room_id, room);
        room_addr
    }
}

impl Handler<JoinRoom> for Lobby {
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Option<Addr<Room>> {
        let mut room = match self.chat_rooms.get(&msg.room_id) {
            Some(room) => room.to_owned(),
            _ => return None,
        };
        let user = self.sessions.get(&msg.user_id);
        if user.is_none() {
            return None;
        }

        let user_addr = user.expect("Error retriving User").to_owned();
        room.add_user(msg.user_id, user_addr);

        Some(room.clone().start())
    }
}
