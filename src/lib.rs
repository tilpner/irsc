#![feature(plugin, custom_derive, slice_patterns)]
#![plugin(regex_macros)]
#![cfg_attr(feature = "lints", plugin(clippy))]

#![deny(warnings)]
#![allow(unused_imports)]

extern crate regex;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate encoding;
extern crate linear_map;

pub mod client;
pub mod color;
pub mod ident;
pub mod callback;
pub mod message;
pub mod command;
pub mod reply;
pub mod event;
pub mod text;

use std::io;
use std::result;
use std::ops::{ Deref, DerefMut };

use openssl::ssl::error::SslError;

use encoding::EncodingRef;

pub use ident::Ident;
pub use message::Message;
pub use command::Command;
pub use reply::Reply;
pub use event::Event;
pub use client::Client;

#[derive(Debug)]
pub enum IrscError {
    Io(io::Error),
    AlreadyConnected,
    NotConnected,
    NotFound,
    Ssl(SslError)
}

impl From<SslError> for IrscError {
    fn from(e: SslError) -> IrscError { IrscError::Ssl(e) }
}

pub struct Result<T>(result::Result<T, IrscError>);

impl<T> Deref for Result<T> {
    type Target = result::Result<T, IrscError>;
    fn deref(&self) -> &result::Result<T, IrscError> { &self.0 }
}

impl<T> DerefMut for Result<T> {
    fn deref_mut(&mut self) -> &mut result::Result<T, IrscError> { &mut self.0 }
}

impl<T> Result<T> { fn inner(self) -> result::Result<T, IrscError> { self.0 } }

pub const DEBUG: bool = cfg!(debug_assertions);
pub static ENCODING: EncodingRef = encoding::all::UTF_8;
