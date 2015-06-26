use std::borrow::{ Borrow, ToOwned };

use command;
use reply;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Command(command::Command),
    Reply(reply::Reply),
    Connected,
    Disconnected
}
