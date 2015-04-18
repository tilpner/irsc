use ident::Ident;

use std::borrow::ToOwned;

#[derive(Clone)]
pub struct Event {
    pub prefix: String,
    pub command: String,
    pub content: Vec<String>
}

pub trait ParseResult {
    fn parse(event: Event) -> Option<Self>;
}

pub const PING: &'static str = "PING";

pub const PRIVMSG: &'static str = "PRIVMSG";

fn join(v: Vec<String>, from: usize) -> String {
    let mut msg = if v[from].chars().next().unwrap() == ':' {
        v[from][1..].to_owned()
    } else { v[from].clone() };
    for m in v.iter().skip(from + 1) {
        msg.push_str(" ");
        msg.push_str(m.trim_right());
    }
    msg
}

pub struct PrivMsg {
    pub from: Ident,
    pub to: String,
    pub content: String
}

impl ParseResult for PrivMsg {
    fn parse(event: Event) -> Option<PrivMsg> {
        let from = Ident::parse(&event.prefix);
        let to = event.content[0].clone();
        match from {
            Some(from) => Some(PrivMsg {
                from: from,
                to: to,
                content: join(event.content, 1)
            }),
            None => None
        }
    }
}
