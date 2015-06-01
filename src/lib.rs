#![feature(plugin, custom_derive, slice_patterns)]
#![plugin(regex_macros)]
#![cfg_attr(feature = "lints", plugin(clippy))]

#![deny(warnings)]
#![allow(unused_imports)]

extern crate regex;
#[macro_use]
extern crate log;
#[cfg(feature = "ssl")]
extern crate openssl;

pub mod client;
pub mod color;
pub mod ident;
pub mod callback;
pub mod message;
pub mod command;
pub mod reply;

use std::io;
use std::result;

#[cfg(feature = "ssl")]
use openssl::ssl::error::SslError;

pub use ident::Ident;
pub use message::{ Message, MsgType };
pub use command::Command;
pub use reply::Reply;
pub use client::Client;

#[derive(Debug)]
pub enum IrscError {
    Io(io::Error),
    AlreadyConnected,
    NotConnected,
    NotFound,
    #[cfg(feature = "ssl")]
    Ssl(SslError)
}

#[cfg(feature = "ssl")]
impl From<SslError> for IrscError {
    fn from(e: SslError) -> IrscError { IrscError::Ssl(e) }
}

pub type Result<T> = result::Result<T, IrscError>;

pub const DEBUG: bool = cfg!(debug_assertions);
