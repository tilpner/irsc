use regex::Regex;

static PATTERN: Regex = regex!(":(.*)!(.*)@(.*)");

#[deriving(Show, Clone)]
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
            nickname: c.at(1).unwrap().into_string(),
            user: c.at(2).unwrap().into_string(),
            host: c.at(3).unwrap().into_string()
        })
    }
}
