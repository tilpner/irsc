#![allow(non_camel_case_types)]

use std::str::{ self, FromStr };
use std::string::{ ToString };
use std::borrow::{ ToOwned };
use std::ops::{ Deref, Range };
use std::fmt;

use linear_map::LinearMap;

use ::IrscError;
use text::{ self, Text, TextSlice };
use ident::Ident;

/// Byte indices, be careful.
/// TODO: more IRCv3 stuff
/// TODO: have MaybeUTF8 enum, try to decode as UTF-8 first,
///       with functions to decode as alternative charsets
/// ircv3.net
#[derive(Clone)]
pub struct Message {
    pub source: Text,
    prefix: Option<Range<u16>>,
    command: Range<u16>,
    content: Vec<Range<u16>>,
    suffix: Option<Range<u16>>,
    // only allocates if tags are present
    tags: LinearMap<Text, Text>
    //pub msg_type: MsgType
}

impl fmt::Debug for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Message")
            .field("source", &self.source)
            .finish()
    }
}

impl Message {
    pub fn new(source: Text, prefix: Option<Range<u16>>, command: Range<u16>, content: Vec<Range<u16>>, suffix: Option<Range<u16>>) -> Message {
        Message {
            source: source,
            prefix: prefix,
            command: command,
            content: content,
            suffix: suffix,
            tags: LinearMap::new()
        }
    }

    fn parse(i: &[u8]) -> Result<Message, IrscError> {
        let len = i.len();
        // Use indices instead of subslices, to store
        // remember, bytes, not chars
        let mut s = 0;

        let prefix = if len >= 1 && i[s] == b':' {
            i[s..].iter().cloned()
                .find(|&b| b == b' ')
                .map(|j| { let n = 1u16..(s + j as usize) as u16; s += j as usize + 1; n })
        } else { None };

        let command = i[s..].iter().cloned()
                        .find(|&b| b == b' ').map(|n| {
            let p = s as u16..(s + n as usize) as u16;
            s += n as usize;
            p
        });

        let mut content = Vec::with_capacity(3);
        let mut suffix = None;
        while s < len - 3 {
            if i[s] == b':' {
                suffix = Some(s as u16 + 1 as u16..i.len() as u16);
                break
            }
            i[s..].iter().cloned()
                .find(|&b| b == b' ').map(|j| {
                if j > 0 {
                    content.push(s as u16..(s + j as usize) as u16);
                    s += j as usize;
                }
            });
            // if s.chars().next() == Some(' ') { s = &s[1..] };
            s += 1;
        }

        /*let msg_type = if suffix.as_ref().map(|s| i[s.start as usize..].as_bytes()[0] == 1
                                               && i[(s.end - 3) as usize..].as_bytes()[0] == 1)
            == Some(true) { MsgType::Ctcp } else { MsgType::Irc };*/

        command.map(move |c|
            Ok(Message::new(
                Text::Raw(i.to_owned()),
                prefix,
                c,
                content,
                // strip \{1} if CTCP message
                // strip \r\n for each line, relying on their existence
                suffix
                /*match msg_type {
                    MsgType::Irc => suffix.map(|s| s.start..s.end - 1),
                    MsgType::Ctcp => suffix.map(|s| s.start + 1..s.end - 2)
                },
                msg_type*/
            ))
        ).unwrap()
    }

    #[allow(unused_assignments)]
    pub fn format<T: Deref<Target=[u8]>>(prefix: Option<T>, command: T, content: Vec<T>, suffix: Option<T>) -> Message {
        let mut s = Vec::with_capacity(512);
        let mut i = 0;

        let mut i_prefix = None;
        if let Some(ref p) = prefix {
            i_prefix = Some((i + 1) as u16..(i + 2 + p.len()) as u16);
            s.push(b':');
            s.push_all(p);
            s.push(b' ');
            i = s.len();
        }

        let i_command = i as u16..(i + command.len()) as u16;
        s.push_all(&command);
        s.push(b' ');
        i = s.len();

        let mut i_content = Vec::new();
        for part in content.iter() {
            i_content.push(i as u16..(i + part.len()) as u16);
            s.push_all(part);
            s.push(b' ');
            i = s.len();
        }

        let mut i_suffix = None;
        if let Some(ref p) = suffix {
            s.push(b':');
            //if let MsgType::Ctcp = msg_type { s.push('\u{1}'); i += 1; }
            let n = i;
            s.push_all(p);
            //if let MsgType::Ctcp = msg_type { s.push('\u{1}'); i += 1; }
            i_suffix = Some(n as u16..(n + p.len()) as u16);
            i = s.len();
        }

        s.push_all(b"\r\n");

        Message::new(Text::Raw(s), i_prefix, i_command, i_content, i_suffix)
    }

    pub fn byte_range(&self, r: &Range<u16>) -> &[u8] {
        &self.source[r.start as usize..r.end as usize]
    }

    pub fn text_range(&self, r: &Range<u16>) -> TextSlice {
        self.source.slice(&(r.start as usize..r.end as usize))
    }

    pub fn string_range(&self, r: &Range<u16>) -> String {
        text::def_lossy_decode(self.byte_range(r))
    }

    pub fn str_range(&self, r: &Range<u16>) -> Option<&str> {
        str::from_utf8(self.byte_range(r)).ok()
    }

    pub fn bytes(&self) -> &[u8] { &*self.source }

    pub fn prefix<'a>(&'a self) -> Option<TextSlice<'a>> {
        self.prefix.as_ref().map(|r| self.text_range(r)) }
    pub fn command<'a>(&'a self) -> TextSlice<'a> {
        self.text_range(&self.command) }
    pub fn content<'a>(&'a self) -> Vec<TextSlice<'a>> {
        self.content.iter().map(|r| self.text_range(&r)).collect() }
    pub fn suffix<'a>(&'a self) -> Option<TextSlice<'a>> {
        self.suffix.as_ref().map(|r| self.text_range(r)) }
    pub fn last<'a>(&'a self) -> Option<TextSlice<'a>> {
        self.suffix().or(self.content.last().map(|l| self.text_range(l))) }
    pub fn elements<'a>(&'a self) -> Vec<TextSlice<'a>> {
        let mut s = self.content(); self.suffix().map(|f| s.push(f)); s }
    pub fn ident(&self) -> Option<Ident> {
        self.prefix().and_then(|p| p.utf8()).and_then(Ident::parse) }
    pub fn is_ctcp(&self) -> bool {
        self.source.get(0) == Some(&1)
     && self.source.get(self.source.length() - 3) == Some(&1)
    }
}

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

#[cfg(test)]
mod test {
    use std::borrow::{ ToOwned };
    use message::{ Message, MsgType };

    #[test]
    fn parse_message1() {
        let b = ":d PRIVMSG You :\u{1}ACTION sends you funny pictures of cats!\u{1}\r\n";
        let b2 = Message::new(
            b.to_owned(),
            Some(1..2),
            3..10,
            vec![11..14],
            Some(17..58),
            MsgType::Ctcp
        );

        assert_eq!(b.parse::<Message>().unwrap(), b2.clone());
        assert_eq!(b2.to_string(), b);
    }

    #[test]
    fn parse_message2() {
        let a = ":a.b.c NOTICE AUTH :*** Looking up your hostname...\r\n";
        // I'm not even kidding...
        let a2 = Message::new(
            a.to_owned(),
            Some(1..6),
            7..13,
            vec![14..18],
            Some(20..52),
            MsgType::Irc
        );
        assert_eq!(a.parse::<Message>().unwrap(), a2.clone());
        assert_eq!(a2.to_string(), a);
    }

    #[test]
    fn format_message() {
        let a = Message::format(Some("a.b.c"), "NOTICE", vec!["AUTH"], Some("*** Looking up your hostname..."), MsgType::Irc);
        let a2 = ":a.b.c NOTICE AUTH :*** Looking up your hostname...\r\n";
        assert_eq!(a.to_string(), a2);
    }
}
