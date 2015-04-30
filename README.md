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

### Planned

- Higher-level wrapper, directly aimed at writing bots
- Lots of tests
- Some documentation (yeah, sure)

## Example

Compiles and works with `rustc 1.1.0-nightly (c4b23aec4 2015-04-29)` and `3e898f8451229bcc4988b40e2edcaec348bf7f79` of this library.

```rust
extern crate irsc;

use std::borrow::ToOwned;
use std::borrow::Cow::*;

use irsc::client::Client;
use irsc::color::bold;
use irsc::*;
use irsc::Command::*;
use irsc::Reply::*;

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn callback(server: &mut Client, msg: &Message) {
    match Command::from_message(msg) {
        Some(PRIVMSG(to, content)) => {
            let from = msg.prefix().and_then(Ident::parse).unwrap();
            let response = match msg.msg_type {
                MsgType::Irc => format!("{} wrote: {}", from.nickname, bold(&content)),
                MsgType::Ctcp => format!("{} emoted: {}", from.nickname, bold(&content["ACTION ".len()..]))
            };
            server.send(PRIVMSG(to, Owned(response))).unwrap();
        },
        _ => ()
    }

    match Reply::from_message(msg) {
        Some(RPL_WELCOME(_)) => {
            server.send(JOIN(vec![Borrowed("#botzoo")], vec![])).unwrap();
        },
        _ => ()
    }
}

fn main() {
    let mut s = Client::new();
    s.connect("irc.mozilla.org".to_owned(), 6667).unwrap();
    s.send(NICK(Borrowed(NAME))).unwrap();
    s.send(USER(Borrowed(NAME), Borrowed("*"), Borrowed("*"), Borrowed(DESC))).unwrap();

    // Dedicate this thread to listening and event processing
    s.listen(callback).unwrap();
}
```
