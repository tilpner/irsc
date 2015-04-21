#![feature(plugin, collections, custom_derive, slice_patterns)]
#![plugin(regex_macros)]

extern crate regex;
#[macro_use]
extern crate log;
extern crate eventual;

pub mod server;
pub mod color;
pub mod ident;
pub mod callback;
pub mod message;
pub mod command;
pub mod reply;

use std::io;
use std::result;

#[derive(Debug)]
pub enum IrscError {
    Io(io::Error),
    AlreadyConnected,
    NotConnected,
    NotFound
}

pub type Result<T> = result::Result<T, IrscError>;

pub const DEBUG: bool = cfg!(not(ndebug));
