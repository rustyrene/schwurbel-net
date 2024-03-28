use actix::prelude::*;
use uuid::Uuid;

use super::ws_conn::ws_conn;

// User connects to lobby
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_id: Uuid,
    pub user_addr: Addr<ws_conn>,
}

// User disconnects from lobby
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: Uuid,
}
