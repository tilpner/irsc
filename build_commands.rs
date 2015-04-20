use std::env;
use std::fs::File;
use std::io::{ Read, Write };
use std::path::Path;
use std::borrow::ToOwned;

#[derive(Debug)]
struct Command {
    command: String,
    params: String,
    doc: String
}

fn main() {
    let mut f = File::open("rfc2812_commands.txt").unwrap();

    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let mut lines = content.lines();
    let mut line = lines.next();

    let mut commands = Vec::new();

    let mut command: Option<&str> = None;
    let mut params: Option<&str> = None;
    let mut doc = String::new();
    while let Some(l) = line {
        // if is new command
        if l.chars().next().map(char::is_whitespace) == Some(false)
            && command.is_some() && params.is_some() {
            commands.push(Command {
                command: command.unwrap().to_owned(),
                params: params.unwrap().to_owned(),
                doc: doc.clone()
            });
            command = None;
            params = None;
            doc.clear();
        }
        if l.trim().starts_with("Command:") {
            command = Some(&l.trim()["Command: ".len()..]);
        } else if l.trim().starts_with("Parameters:") {
            params = Some(&l.trim()["Parameters: ".len()..]);
        }
        doc.push_str(l);
        doc.push_str("\n");

        line = lines.next();
    }

    println!("pub enum Command {{");
    for c in commands {
        for l in c.doc.lines() {
            println!("    /// {}", l);
        }

        println!("    {}({}),\n", c.command, c.params);
    }
    println!("}}");
}
