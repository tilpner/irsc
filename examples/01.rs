#![allow(unstable)]
#![feature(plugin, slicing_syntax)]
#![plugin(regex_macros)]

extern crate irsc;

use std::borrow::ToOwned;

use std::sync::{Once, ONCE_INIT};

use irsc::server::Server;
use irsc::color::bold;
use irsc::message;
use irsc::message::{ Message, Command };

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

static START: Once = ONCE_INIT;

fn callback(server: &mut Server, msg: &Message) {
    match Command::from_message(msg) {
        Some(Command::PrivMsg { from, content, .. }) => {
            let response = format!("You wrote: {}", bold(&content));
            server.msg(&from.unwrap(), &response).unwrap();
        },
        _ => {}
    }

    /*
        "001" => {
            START.call_once(|| {
                server.msg("Nalfon", "Hey, I'm poking you! *pokes you*").unwrap();
                //server.msg("Xasin", "Hey, I'm poking you! *pokes you*").unwrap();
            })
        },
        _ => ()
    }*/
}

fn main() {
    let mut s = Server::new();
    s.connect("irc.furnet.org".to_owned(), 6667).unwrap();
    s.nick(NAME).unwrap();
    s.user(NAME, "*", "*", DESC).unwrap();
    s.join("#botzoo").unwrap();

    // Dedicate this thread to listening and event processing
    s.listen(&[callback]).unwrap();
}
