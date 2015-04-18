irsc
[![travis-ci.org](https://travis-ci.org/tilpner/irsc.svg?branch=master)](https://travis-ci.org/tilpner/irsc)
[![crates.io](http://meritbadge.herokuapp.com/irsc)](https://crates.io/crates/irsc)
=========

*This repository contains code that has not been properly tested yet, continue
at the risk of doing stupid things while discovering parts of this library
don't work.*

## Introduction

Want to build an IRC bot with low resource consumption? You might want to have a look at this library (maybe later, though).

This library is supposed to be a thin layer over the IRC protocol, doing all the network IO and event parsing for you. Right now, it only works, nothing more.

## Example

Compiles and works with `rustc 0.13.0-nightly (3327ecca4 2014-11-01 22:41:48 +0000)` and `11b12ad` of this library.

```rust
#![feature(globs, slicing_syntax)]

extern crate irsc;

use irsc::server::Server;
use irsc::color::bold;
use irsc::event;
use irsc::event::{ Event, ParseResult, PrivMsg };

static NAME: &'static str = "rusticbot";
static DESC: &'static str = "A bot, written in Rust.";

fn callback(arg: (Server, Event)) {
    let (mut server, event) = arg;
    match event.command[] {
        event::PRIVMSG => {
            let privmsg: PrivMsg = ParseResult::parse(event).unwrap();
            let response = format!("You wrote: {}", bold(privmsg.content[]));
            server.msg(privmsg.from.nickname[], response[]).unwrap();
        },
        _ => ()
    }
}

fn main() {
    let mut s = Server::new();
    s.connect("irc.freenode.org".into_string(), 6667).unwrap();
    s.nick(NAME).unwrap();
    s.user(NAME, "*", "*", DESC).unwrap();
    s.join("#botzoo").unwrap();

    s.msg("flan3002", "Hey there! You should probably change the nick in this README!").unwrap();

    s.events.lock().register(&callback);

    // Dedicate this thread to listening and event processing
    s.listen().unwrap();
}
```
