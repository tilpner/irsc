use std::borrow::{ Borrow, ToOwned };

use command;
use reply;

#[derive(Debug, Clone, PartialEq)]
pub enum Event<'a> {
    Command(command::Command<'a>),
    Reply(reply::Reply<'a>),
    Connected,
    Disconnected
}

impl<'a> Event<'a> {
    pub fn to_static(&self) -> Event<'static> {
        use Event::*;
        match self {
            &Command(ref c) => Command(c.to_static()),
            &Reply(ref r) => Reply(r.to_static()),
            &Connected => Connected,
            &Disconnected => Disconnected
        }
    }
}
