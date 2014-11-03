#![feature(globs, slicing_syntax)]

extern crate irsc;

use irsc::server::Server;
use irsc::color::bold;
use irsc::event;
use irsc::event::{ Event, ParseResult, PrivMsg };

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn callback(arg: (Server, Event)) {
    let (mut server, event) = arg;
    match event.command[] {
        event::PRIVMSG => {
            let privmsg: PrivMsg = ParseResult::parse(event).unwrap();
            let response = format!("You wrote: {}", bold(privmsg.content[]));
            server.msg(privmsg.from.nickname[], response[]).unwrap();
        },
        _ => ()
    }
}

fn main() {
    let mut s = Server::new();
    s.connect("irc.freenode.org".into_string(), 6667).unwrap();
    s.nick(NAME).unwrap();
    s.user(NAME, "*", "*", DESC).unwrap();
    s.join("#botzoo").unwrap();

    s.msg("flan3002", "Hey!").unwrap();

    s.events.lock().register(&callback);

    // Dedicate this thread to listening and event processing
    s.listen().unwrap();
}
