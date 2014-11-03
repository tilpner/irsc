use ident::Ident;

#[deriving(Clone)]
pub struct Event {
    pub prefix: String,
    pub command: String,
    pub content: String
}

pub trait ParseResult {
    fn parse(event: Event) -> Option<Self>;
}

pub const PRIVMSG: &'static str = "PRIVMSG";

pub struct PrivMsg {
    pub from: Ident,
    pub to: String,
    pub content: String
}

impl ParseResult for PrivMsg {
    fn parse(event: Event) -> Option<PrivMsg> {
        let from = Ident::parse(event.prefix[]);
        match from {
            Some(from) => Some(PrivMsg {
                from: from,
                content: event.content
            }),
            None => None
        }
    }
}
