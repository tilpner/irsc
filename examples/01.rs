extern crate irsc;
extern crate env_logger;
#[cfg(feature = "ssl")]
extern crate openssl;

use std::borrow::ToOwned;
use std::borrow::Cow::*;

use irsc::client::Client;
use irsc::color::bold;
use irsc::*;
use irsc::Command::*;
use irsc::Reply::*;

#[cfg(feature = "ssl")]
use openssl::ssl::{ Ssl, SslContext, SslMethod };

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

            // only send to channels, to prevent recursion when we are pm'ed
            // technically, there are other prefixes than '#', but ignoring them is fine
            if to.starts_with("#") {
                server.send(PRIVMSG(to, Owned(response))).unwrap();
            }
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

#[cfg(feature = "ssl")]
fn connect(s: &mut Client) {
    let ssl = Ssl::new(&SslContext::new(SslMethod::Tlsv1).unwrap()).unwrap();
    s.connect_ssl("irc.mozilla.org".to_owned(), 6697, ssl).unwrap();
}

#[cfg(not(feature = "ssl"))]
fn connect(s: &mut Client) {
    s.connect("irc.mozilla.org".to_owned(), 6667).unwrap();
}

fn main() {
    env_logger::init().unwrap();
    let mut s = Client::new();
    connect(&mut s);
    s.send(NICK(Borrowed(NAME))).unwrap();
    s.send(USER(Borrowed(NAME), Borrowed("*"), Borrowed("*"), Borrowed(DESC))).unwrap();

    // Dedicate this thread to listening and event processing
    s.listen(callback).unwrap();
}
