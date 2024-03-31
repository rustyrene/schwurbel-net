use actix::prelude::*;
use uuid::Uuid;

use super::{chat_room::Room, ws_conn::WsConn};

// User connects to lobby
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: Uuid,
    pub user_addr: Addr<WsConn>,
}

// User disconnects from lobby
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: Uuid,
}

// Server Message to client
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

// ClientMessage to Server/Chatroom
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub sender_id: Uuid,
    pub message: String,
}

// ClientMessage to Server/Chatroom
#[derive(Message)]
#[rtype(result = "Addr<Room>")]
pub struct CreateRoom {
    pub creater_id: Uuid,
}

// Join an existing room
#[derive(Message)]
#[rtype(result = "Option<Addr<Room>>")]
pub struct JoinRoomLobby {
    pub user_id: Uuid,
    pub room_id: Uuid,
}

// Join an existing room
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub user_id: Uuid,
    pub user_addr: Addr<WsConn>,
}

// List all existing rooms
#[derive(Message)]
#[rtype(result = "()")]
pub struct ListRooms {
    pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub user_id: Uuid,
    pub room_id: Uuid,
}
