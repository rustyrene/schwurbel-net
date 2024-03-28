use actix::prelude::*;

pub struct Room {}

impl Actor for Room {
    type Context = Context<Self>;
}
