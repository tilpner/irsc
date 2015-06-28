extern crate irsc;
extern crate openssl;

use irsc::*;
use irsc::Command::*;
use irsc::Reply::*;

use openssl::ssl::{ Ssl, SslContext, SslMethod };

fn main() {
    let mut s = OwnedClient::new();
    let ssl = Ssl::new(&SslContext::new(SslMethod::Tlsv1).unwrap()).unwrap();
    s.connect_ssl("irc.rizon.net", 6697, ssl);
    s.register("irsc", "irsc", "Testing for kori", None);

    let mut shared = s.into_shared();
    let _ = shared.commands()
        .map(|(mut cl, msg, c)| {
            if let PRIVMSG(to, content) = c {
                let from = msg.ident().unwrap();
                let response = format!("{} told me: {}", from.nickname, color::bold(&content));

                // only send to global channels, to prevent recursion when we are pm'ed
                // technically, there are other prefixes than '#', but ignoring them is fine
                if to.starts_with("#") && content.starts_with("irsc") {
                    cl.msg(&to, &response);
                }
            }
        });

    let _b = shared.replies()
        .map(|(mut cl, _msg, r)| {
            if let RPL_WELCOME(_) = r {
                cl.join("#meep!", None);
            }
        });

    // Dedicate this thread to listening and event processing
    shared.listen_with_events();
}
