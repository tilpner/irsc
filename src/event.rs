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
