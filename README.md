# irsc

[![travis-ci.org](https://travis-ci.org/tilpner/irsc.svg?branch=master)](https://travis-ci.org/tilpner/irsc)
[![crates.io](http://meritbadge.herokuapp.com/irsc)](https://crates.io/crates/irsc)

*This repository contains code that has not been properly tested yet, continue
at the risk of doing stupid things while discovering parts of this library
don't work.*

## Overview

Want to build an IRC bot with low resource consumption? You might want to have a look at this library (maybe later, though).

This library is supposed to be a thin layer over the IRC protocol, doing all the network IO and event parsing for you. Right now, it only works, nothing more.

## Features

- Semi-complete implementation of [RFC2812](http://tools.ietf.org/html/rfc2812)
- Some CTCP support
- SSL for connections

### Planned

- Higher-level wrapper, directly aimed at writing bots
- Lots of tests
- Some documentation (yeah, sure)

## Example

Compiles and tested with `rustc 1.2.0-nightly (8937ec100 2015-06-15)` and `63838165c31397fec199bf99c96497a1169c4d52` of this library.

Run with

    cargo run --example 01 --features ssl

and join [#botzoo on irc.mozilla.org](http://irc.lc/mozilla/botzoo).

```rust
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
```
