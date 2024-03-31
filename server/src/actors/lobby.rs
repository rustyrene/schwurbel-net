use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{
    chat_room::Room,
    messages::{
        ClientMessage, Connect, CreateRoom, Disconnect, JoinRoom, JoinRoomLobby, LeaveRoom,
        ListRooms, Message,
    },
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

impl Handler<CreateRoom> for Lobby {
    type Result = Addr<Room>;

    fn handle(&mut self, msg: CreateRoom, _ctx: &mut Self::Context) -> Addr<Room> {
        let creater_addr = self
            .sessions
            .get(&msg.creater_id)
            .expect("Session not found");

        let mut room = Room::new();
        let room_id = room.id.clone();
        println!("{}", room_id);
        room.add_user(msg.creater_id, creater_addr.clone());
        let room_addr = room.start();

        for (user_id, user_addr) in self.sessions.clone() {
            if user_id != msg.creater_id {
                user_addr.do_send(Message(format!("/created {}", room_id.clone())));
            } else {
                user_addr.do_send(Message(format!("/creator {}", room_id.clone())));
            }
        }

        self.chat_rooms.insert(room_id, room_addr.clone());

        room_addr
    }
}

impl Handler<JoinRoomLobby> for Lobby {
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: JoinRoomLobby, _ctx: &mut Self::Context) -> Option<Addr<Room>> {
        let room_addr = match self.chat_rooms.get(&msg.room_id) {
            Some(chat_room) => chat_room,
            None => return None,
        };

        let user = self.sessions.get(&msg.user_id);
        if user.is_none() {
            return None;
        }

        room_addr.do_send(JoinRoom {
            user_addr: user.unwrap().clone(),
            user_id: msg.user_id,
        });

        Some(room_addr.clone())
    }
}

impl Handler<ListRooms> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ListRooms, _ctx: &mut Self::Context) {
        let user = self.sessions.get(&msg.user_id);
        if user.is_none() {
            println!("User not in sessions");
        }

        let room_ids = self
            .chat_rooms
            .keys()
            .map(|uuid| uuid.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        user.unwrap()
            .do_send(Message(format!("/list {}", room_ids)));
    }
}

impl Handler<LeaveRoom> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) {
        let room = match self.chat_rooms.get(&msg.room_id) {
            Some(room) => room,
            None => {
                self.sessions
                    .get(&msg.user_id)
                    .unwrap()
                    .do_send(Message("/error Chatroom not found".to_string()));
                return;
            }
        };

        room.do_send(LeaveRoom {
            user_id: msg.user_id,
            room_id: msg.room_id,
        });
    }
}
