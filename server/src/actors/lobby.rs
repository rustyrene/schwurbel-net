use actix::prelude::*;

use super::messages::{Connect, Disconnect};

pub struct Lobby {}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _ctx: &mut Self::Context) {
        println!("Disconnecting!");
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Connecting");
    }
}
