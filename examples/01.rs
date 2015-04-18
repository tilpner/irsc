extern crate irsc;

use irsc::server::Server;
use irsc::color::bold;
use irsc::event;
use irsc::event::{ Event, ParseResult, PrivMsg };

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn callback(arg: (Server, Event)) {
    let (mut server, event) = arg;
    match &*event.command {
        event::PRIVMSG => {
            let privmsg: PrivMsg = ParseResult::parse(event).unwrap();
            let response = format!("You wrote: {}", bold(&privmsg.content));
            server.msg(&privmsg.from.nickname, &response).unwrap();
        },
        _ => ()
    }
}

fn entry() -> Result<(), String> {
    let mut s = Server::new();
    s.connect("irc.freenode.org".to_string(), 6667).unwrap();

    try!(s.nick(NAME));
    try!(s.user(NAME, "*", "*", DESC));
    try!(s.join("#botzoo"));

    try!(s.msg("flan3002", "Hey, I'm your example bot!"));

    s.events.lock().unwrap().register(&(callback as fn((Server,Event))));

    // Dedicate this thread to listening and event processing
    Ok(try!(s.listen()))
}

fn main() {
    if let Err(err) = entry() {
        println!("error: {}", err)
    }
}
