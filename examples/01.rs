extern crate irsc;

use std::borrow::ToOwned;
use std::borrow::Cow::*;

use irsc::client::Client;
use irsc::color::bold;
use irsc::*;
use irsc::Command::*;
use irsc::Reply::*;

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn callback(server: &mut Client, msg: &Message) {
    match Command::from_message(msg) {
        Some(PRIVMSG(to, content)) => {
            let from = msg.prefix().and_then(Ident::parse).unwrap();
            let response = match msg.msg_type {
                MsgType::Irc => format!("{} wrote: {}", from.nickname, bold(&content)),
                MsgType::Ctcp => format!("{} emoted: {}", from.nickname, bold(&content["ACTION ".len()..]))
            };
            server.send(PRIVMSG(to, Owned(response))).unwrap();
        },
        _ => ()
    }

    match Reply::from_message(msg) {
        Some(RPL_WELCOME(_)) => {
            server.send(JOIN(vec![Borrowed("#botzoo")], vec![])).unwrap();
        },
        _ => ()
    }
}

fn main() {
    let mut s = Client::new();
    s.connect("irc.mozilla.org".to_owned(), 6667).unwrap();
    s.send(NICK(Borrowed(NAME))).unwrap();
    s.send(USER(Borrowed(NAME), Borrowed("*"), Borrowed("*"), Borrowed(DESC))).unwrap();

    // Dedicate this thread to listening and event processing
    s.listen(callback).unwrap();
}
