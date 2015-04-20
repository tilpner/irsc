#![allow(non_camel_case_types)]

use std::str::FromStr;
use std::string::{ ToString };
use std::borrow::{ ToOwned };
use std::ops::Range;

use ::IrscError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MsgType {
    /// Plain old IRC messages, as defined in [rfc2812][rfc]
    /// rfc: http://tools.ietf.org/html/rfc2812
    Irc,
    /// Ctcp messages, wrapped in \u{1}
    Ctcp
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub source: String,
    prefix: Option<Range<u16>>,
    command: Range<u16>,
    content: Vec<Range<u16>>,
    suffix: Option<Range<u16>>,
    pub msg_type: MsgType
}

impl Message {
    pub fn new(source: String, prefix: Option<Range<u16>>, command: Range<u16>, content: Vec<Range<u16>>, suffix: Option<Range<u16>>, msg_type: MsgType) -> Message {
        Message {
            source: source,
            prefix: prefix,
            command: command,
            content: content,
            suffix: suffix,
            msg_type: msg_type
        }
    }

    #[allow(unused_assignments)]
    pub fn format(prefix: Option<&str>, command: &str, content: Vec<&str>, suffix: Option<&str>, msg_type: MsgType) -> Message {
        let mut s = String::with_capacity(512);
        let mut i = 0;

        let mut i_prefix = None;
        if let Some(ref p) = prefix {
            i_prefix = Some((i + 1) as u16..(i + 2 + p.len()) as u16);
            s.push(':');
            s.push_str(p);
            s.push(' ');
            i += 2 + p.len();
        }

        let i_command = i as u16..(i + command.len()) as u16;
        s.push_str(command);
        s.push(' ');
        i += 1 + command.len();

        let mut i_content = Vec::new();
        for part in content.iter() {
            i_content.push(i as u16..(i + part.len()) as u16);
            s.push_str(part);
            s.push(' ');
            i += 1 + part.len();
        }

        let mut i_suffix = None;
        if let Some(ref p) = suffix {
            s.push(':');
            if let MsgType::Ctcp = msg_type { s.push('\u{1}'); i += 1; }
            let n = i;
            s.push_str(p);
            if let MsgType::Ctcp = msg_type { s.push('\u{1}'); i += 1; }
            i_suffix = Some(n as u16..(n + p.len()) as u16);
            i += 1 + p.len();
        }

        s.push_str("\r\n");
        i += 2;

        Message::new(s, i_prefix, i_command, i_content, i_suffix, msg_type)
    }

    pub fn range(&self, r: &Range<u16>) -> &str {
        self.source.slice_chars(r.start as usize, r.end as usize)
    }

    pub fn prefix(&self) -> Option<&str> { self.prefix.as_ref().map(|r| self.range(r)) }
    pub fn command(&self) -> &str { self.range(&self.command) }
    pub fn content(&self) -> Vec<&str> { self.content.iter().map(|r| self.range(&r)).collect() }
    pub fn suffix(&self) -> Option<&str> { self.suffix.as_ref().map(|r| self.range(r)) }
}

impl FromStr for Message {
    type Err = IrscError;
    fn from_str(i: &str) -> Result<Message, IrscError> {
        info!("Attempting to parse message: {}", i);
        let len = i.len();
        let mut s = 0;

        let prefix = if len >= 1 && i[s..].chars().next() == Some(':') {
            i[s..].find(' ').map(|i| 1u16..i as u16)
        } else { None };

        let command = i[s..].find(' ').map(|n| {
            let p = s as u16..n as u16;
            s = n;
            p
        });

        // TODO: Parse last non-suffix argument as suffix if no suffix
        // with colon is available.
        let mut content = Vec::with_capacity(15);
        let mut suffix = None;
        while i[s..].len() > 0 {
            if i[s..].chars().next() == Some(':') {
                suffix = Some(s as u16 + 1 as u16..i.len() as u16);
                break
            }
            i[s..].find(' ').map(|i| {
                if i > 0 {
                    content.push(s as u16..(s + i) as u16);
                    s = i;
                }
            });
            // if s.chars().next() == Some(' ') { s = &s[1..] };
            s += 1;
        }

        let msg_type = if suffix.as_ref()
            .and_then(|s| i[s.start as usize..].chars().next()) == Some('\u{1}') { MsgType::Ctcp } else { MsgType::Irc };

        command.map(move |c|
            Ok(Message::new(
                i.to_owned(),
                prefix,
                c,
                content,
                // strip \{1} if CTCP message
                // strip \r\n for each line, relying on their existence
                match msg_type {
                    MsgType::Irc => suffix.map(|s| s.start..s.end - 2),
                    MsgType::Ctcp => suffix.map(|s| s.start + 1..s.end - 3)
                },
                msg_type
            ))
        ).unwrap()

    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        self.source.clone()
        /*
        let mut s = String::with_capacity(512);
        if let Some(ref p) = self.prefix() {
            s.push(':');
            s.push_str(p);
            s.push(' ');
        }

        s.push_str(self.command());
        s.push(' ');

        for part in self.content.iter() {
            s.push_str(self.range(&part));
            s.push(' ');
        }

        if let Some(ref p) = self.suffix() {
            s.push(':');
            if let MsgType::Ctcp = self.msg_type { s.push('\u{1}') }
            s.push_str(p);
            if let MsgType::Ctcp = self.msg_type { s.push('\u{1}') }
        }

        s.push_str("\r\n");
        s*/
    }
}

/*
pub trait Command {
    fn name(&self) -> String;
    fn to_message(&self) -> Message;
    fn from_message(msg: &Message) -> Option<Self>
}

macro_rules! command (
    ($name: ident { $($field: ident: $t: ty),* } to $to_msg: expr; from $from_msg: expr;) => (
        pub struct $name {
            $(pub $field: $t),*
        }

        impl Command for $name {
            fn name(&self) -> String { stringify!($name).to_owned() }
            fn to_message(&self) -> Message { ($to_msg)(self) }
            fn from_message(msg: &Message) -> Option<$name> { ($from_msg)(msg) }
        }
    )
);*/

/*
command!(Pass { password: String }
         to |&:s: &Pass| Message::new(None, "PASS".to_owned(), Vec::new(), Some(s.password.clone()));
         from |&:msg: &Message| msg.clone().suffix.map(|s| Pass { password: s }););

command!(Ping { server1: Option<String>, server2: Option<String> }
         to |&:s :&Ping| {
             let mut v = Vec::new();
             if let Some(ref s) = s.server1 { v.push(s.clone()) }
             if let Some(ref s) = s.server2 { v.push(s.clone()) }
             Message::new(None, "PING".to_owned(), v, None)
         };
         from |&:msg: &Message| {
             let mut c = msg.content.clone();
             Some(Ping { server2: c.pop(), server1: c.pop() }) }; );

command!(Pong { server1: Option<String>, server2: Option<String> }
         to |&:s: &Pong| {
             let mut v = Vec::new();
             if let Some(ref s) = s.server1 { v.push(s.clone()) }
             if let Some(ref s) = s.server2 { v.push(s.clone()) }
             Message::new(None, "PONG".to_owned(), v, None)
         };
         from |&:msg: &Message| {
             let mut c = msg.content.clone();
             Some(Pong { server2: c.pop(), server1: c.pop() })
         }; );

command!(PrivMsg { from: Option<String>, to: String, content: String }
         to |&:s: &PrivMsg| {
             Message::new(s.from.clone(), "PRIVMSG".to_owned(), vec![s.to.clone()], Some(s.content.clone()))
         };
         from |&:msg: &Message| {
             msg.content.clone().pop().map(
                 |c| PrivMsg {
                     from: msg.prefix.clone(),
                     to: c,
                     content: msg.suffix.clone().unwrap_or(String::new())
                 })
         }; );
*/

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Away,
    Invisible,
    Wallops,
    Restricted,
    Operator,
    LocalOperator,
    ServerNotices,
    Custom(String)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SetMode {
    Plus,
    Minus
}

/// If you hoped it couldn't get any uglier... I'm sorry, it does.
/// Why a giant match? API.
///
/// I tried structuring it as a bunch of structs that impl a `Command` trait,
/// but the user would have to use Any and lots of cats. Also, extensibility isn't
/// really a goal; the IRC protocol doesn't seem to evolve very fast.
///
/// Granted, I *could* have used a phf-map to map to functions to parse this, which
/// - while more readable - shouldn't have resulted in performance gains.
///
/// Please don't cry.

/*pub fn join(v: Vec<String>, from: usize) -> String {
    let mut msg = if v[from].chars().next().unwrap() == ':' {
        v[from][1..].to_owned()
    } else { v[from].clone() };
    for m in v.iter().skip(from + 1) {
        msg.push_str(" ");
        msg.push_str(m.trim_right());
    }
    msg
}*/

/*pub struct PrivMsg {
    pub from: Ident,
    pub to: String,
    pub content: String
}

impl ParseResult for PrivMsg {
    fn parse(message: Message) -> Option<PrivMsg> {
        let from = Ident::parse(message.prefix.unwrap()[]);
        let to = message.content[0].clone();
        match from {
            Some(from) => Some(PrivMsg {
                from: from,
                to: to,
                content: join(message.content, 1)
            }),
            None => None
        }
    }
}*/

#[cfg(test)]
mod test {
    use std::borrow::{ ToOwned };
    use message::{ Message, MsgType };

    #[test]
    fn parse_message() {
        /*let a = ":a.b.c NOTICE AUTH :*** Looking up your hostname...\r\n";
        // I'm not even kidding...
        let a2 = Message::new(
            a.to_owned(),
            Some(Cow::Owned("a.b.c".to_owned())),
            Cow::Owned("NOTICE".to_owned()),
            vec![Cow::Owned("AUTH".to_owned())],
            Some(Cow::Owned("*** Looking up your hostname...".to_owned())),
            MsgType::Irc
        );
        assert_eq!(a.parse::<Message>().unwrap(), a2.clone());
        assert_eq!(a2.to_string(), a);*/

        let b = ":d PRIVMSG You :\u{1}ACTION sends you funny pictures of cats!\u{1}\r\n";
        let b2 = Message::new(
            b.to_owned(),
            Some(1..2),
            3..10,
            vec![11..14],
            Some(17..57),
            MsgType::Ctcp
        );

        assert_eq!(b.parse::<Message>().unwrap(), b2.clone());
        assert_eq!(b2.to_string(), b);
    }
}
