#![feature(globs, phase, slicing_syntax, macro_rules, unboxed_closures)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

#[phase(plugin, link)]
extern crate log;

pub mod server;
pub mod color;
pub mod ident;
pub mod callback;
pub mod event;
