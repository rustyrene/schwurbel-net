use actix::prelude::*;
use uuid::Uuid;

use super::ws_conn::WsConn;

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
#[rtype(result = "()")]
pub struct CreateRoom {
    pub creater_id: Uuid,
}
