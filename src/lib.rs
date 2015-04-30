#![feature(plugin, custom_derive, slice_patterns)]
#![plugin(regex_macros)]

#![deny(warnings)]
#![allow(unused_imports)]

extern crate regex;
#[macro_use]
extern crate log;

pub mod client;
pub mod color;
pub mod ident;
pub mod callback;
pub mod message;
pub mod command;
pub mod reply;

use std::io;
use std::result;

pub use ident::Ident;
pub use message::{ Message, MsgType };
pub use command::Command;
pub use reply::Reply;

#[derive(Debug)]
pub enum IrscError {
    Io(io::Error),
    AlreadyConnected,
    NotConnected,
    NotFound
}

pub type Result<T> = result::Result<T, IrscError>;

pub const DEBUG: bool = cfg!(not(ndebug));
