#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;

#[macro_use]
extern crate log;

pub mod server;
pub mod color;
pub mod ident;
pub mod callback;
pub mod event;
