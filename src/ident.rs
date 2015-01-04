use regex::Regex;
use std::borrow::ToOwned;

static PATTERN: Regex = regex!(":(.*)!(.*)@(.*)");

#[derive(Show, Clone)]
pub struct Ident {
    pub nickname: String,
    pub user: String,
    pub host: String
}

impl Ident {
    pub fn parse(s: &str) -> Option<Ident> {
        let c = match PATTERN.captures(s) {
            Some(c) => c,
            None => return None
        };
        Some(Ident {
            nickname: c.at(1).unwrap().to_owned(),
            user: c.at(2).unwrap().to_owned(),
            host: c.at(3).unwrap().to_owned()
        })
    }
}
