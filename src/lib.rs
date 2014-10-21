#![feature(globs, phase, slicing_syntax, macro_rules)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub mod server;
pub mod events;
pub mod color;
pub mod ident;
