#![feature(globs, slicing_syntax)]

extern crate irsc;

use std::borrow::ToOwned;

use std::sync::{Once, ONCE_INIT};

use irsc::server::Server;
use irsc::color::bold;
use irsc::event;
use irsc::event::{ Event, ParseResult, PrivMsg };

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

static START: Once = ONCE_INIT;

fn callback(arg: &(Server, Event)) {
    let (mut server, event) = arg;
    match event.command[] {
        event::PRIVMSG => {
            let privmsg: PrivMsg = ParseResult::parse(event).unwrap();
            let response = format!("You wrote: {}", bold(privmsg.content[]));
            server.msg(privmsg.from.nickname[], response[]).unwrap();
        },
        event::MODE => {
            START.doit(|| {
                server.msg("Syna", "Hey, I'm poking you! *pokes you*").unwrap();
                //server.msg("Xasin", "Hey, I'm poking you! *pokes you*").unwrap();
            })
        },
        _ => ()
    }
}

fn main() {
    let mut s = Server::new();
    s.connect("irc.tulpa.info".to_owned(), 6667).unwrap();
    s.nick(NAME).unwrap();
    s.user(NAME, "*", "*", DESC).unwrap();
    s.join("#botzoo").unwrap();

    s.events.lock().register(&(callback as fn(&(Server, Event))));

    // Dedicate this thread to listening and event processing
    s.listen().unwrap();
}
