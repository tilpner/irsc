extern crate irsc;
extern crate openssl;
extern crate env_logger;

use irsc::*;
use irsc::Command::*;
use irsc::Reply::*;

use openssl::ssl::{ Ssl, SslContext, SslMethod };

// Here should be the constants of this bot, like the nickname or description,
// to avoid repetition. However, for better readability, the constants have been
// inlined below.

fn main() {
    // If the environment variable RUST_LOG is set to "info",
    // irsc will log incoming and outgoing data in raw form.
    env_logger::init().ok().expect("Failed to initialise env_logger");

    let mut s = OwnedClient::new();
    // Try to use Tlsv1 to connect. This might fail, depending on your version of
    // OpenSSL. This example does not try with other methods on failure.
    let ssl = Ssl::new(&SslContext::new(SslMethod::Tlsv1).unwrap()).unwrap();

    // Connect using the newly constructed Ssl configuration.
    // If Ssl is not desired, use .connect(...) without the ssl argument.
    s.connect_ssl("irc.mozilla.org", 6697, ssl);
    // Send the USER and NICK message in one go,
    // <nick> <user> <description> <optional password>
    s.register("irsc02", "irsc", "Example bot 02", None);

    // The client must be accessible from anywh when using the Event API.
    // .into_shared() will convert the previously owned client into a wrapper.
    let mut shared = s.into_shared();

    // `shared.commands()` will return a carboxyl stream of tuples of the form
    // (SharedClient, Message, Command), representing the incoming Commands.
    // Event streams are not lazy, but they must still be alive when the events happen,
    // in order to process them. To keep them alive (prevent dropping), we assing them
    // to local variables. Make sure to not ignore (variable name `_`) them, as that
    // drops them as well. The leading underscore will avoid unused-variable warnings.
    let _a = shared.commands()
        .map(|(mut cl, msg, c)| {
            // PRIVMSGs are the most common way to talk, they are used in queries but also
            // to talk in channels (despite the "PRIV"). This if-let makes sure we only handle
            // these PRIVMSG events for now, and allows us to access PRIVMSG-specific information,
            // like the addressee of the message and its content.
            if let PRIVMSG(to, content) = c {
                // The ident is unique to the origin of this message, and can be used
                // to retrieve the nickname of the sender. This example assumes all
                // hostmasks are well formed and always present.
                let from = msg.ident().unwrap();
                // This example will echo the input if the bot was mentioned.
                // Example:
                //   tilpner told me: irsc - Foo!
                // if "tilpner" wrote "irsc - Foo!" before.
                let response = format!("{} told me: {}", from.nickname, color::bold(&content));

                // Only send to global channels, to prevent recursion when we are pm'ed.
                // Technically, there are other prefixes than '#', but ignoring them is fine here.
                // Also, we only reply if we were mentioned at the start of the message.
                if to.starts_with("#") && content.starts_with("irsc") {
                    // `to` is not the nick who mentioned us, but the channel we were mentioned in,
                    // This will send our `response` to that channel.
                    cl.msg(&to, &response);
                }
            }
        });

    // `shared.replies()` will return a carboxyl stream of tuples of the form
    // (SharedClient, Message, Reply), representing the incoming Replies.
    // Again, make sure to keep the mapped stream alive.
    let _b = shared.replies()
        .map(|(mut cl, _msg, r)| {
            // Logging into the IRC server might take some seconds. Some libraries solve
            // this by having a fixed timeout after they logged in, before they continue
            // to e.g. join channels or identify with the services.
            // There is a more precise way, namely waiting for the RPL_WELCOME event (001),
            // that is sent by the IRCd after we've connected successfully.
            if let RPL_WELCOME(_) = r {
                // After we've connected successfully, we join a channel
                // without providing a password.
                cl.join("#meep!", None);
            }
        });

    // Dedicate this thread to listening and event processing.
    // This method will only return after the connection has been closed or an
    // error was encountered, which is why it should either listen in a new thread,
    // or all necessary setup must be done prior to calling this method.
    shared.listen_with_events();
}
