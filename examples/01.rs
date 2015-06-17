extern crate irsc;
extern crate env_logger;
extern crate openssl;

use irsc::color::bold;
use irsc::*;
use irsc::Command::*;
use irsc::Reply::*;

use openssl::ssl::{ Ssl, SslContext, SslMethod };

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn callback(server: &mut Client, msg: &Message, event: Option<Event>) {
    match event {
        Some(Event::Command(PRIVMSG(to, content))) => {
            let from = msg.ident().unwrap();
            let response = match msg.msg_type {
                MsgType::Irc => format!("{} wrote: {}", from.nickname, bold(&content)),
                MsgType::Ctcp => format!("{} emoted: {}", from.nickname,
                                         bold(&content["ACTION ".len()..]))
            };

            // only send to channels, to prevent recursion when we are pm'ed
            // technically, there are other prefixes than '#', but ignoring them is fine
            if to.starts_with("#") {
                server.msg(&to, &response);
            }
        },
        Some(Event::Reply(RPL_WELCOME(_))) => {
            server.join("#botzoo", None);
        },
        _ => ()
    }
}

fn main() {
    env_logger::init().unwrap();
    let mut s = Client::new();
    let ssl = Ssl::new(&SslContext::new(SslMethod::Tlsv1).unwrap()).unwrap();
    s.connect_ssl("irc.mozilla.org", 6697, ssl);
    s.register(NAME, NAME, DESC);

    // Dedicate this thread to listening and event processing
    s.listen(Some(callback));
}
