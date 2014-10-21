#![feature(globs, slicing_syntax)]

extern crate irsc;

use irsc::server::Server;
use irsc::events::*;
use irsc::color::bold;

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn main() {
    let mut s = Server::new("irc.freenode.org".into_string(), 6667);
    let events = s.events();
    s.connect().unwrap();
    s.nick(NAME).unwrap();
    s.user(NAME, "*", "*", DESC).unwrap();
    s.join("#botzoo").unwrap();

    s.msg("flan3002", "Hey!").unwrap();

    for e in events.iter() {
        match e {
            RplWelcome(welcome) => {
                println!("{}", welcome)
            },
            PrivMsg(from, _to, msg) => {
                let response = format!("You wrote: {}", bold(msg[]));
                s.msg(from.nickname[], response[]).unwrap();
            }
            _ => ()
        }
    }
}
