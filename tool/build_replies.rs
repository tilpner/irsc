use std::fs::File;
use std::io::{ Read };
use std::borrow::ToOwned;

#[derive(Debug)]
struct Reply {
    number: String,
    reply: String,
    doc: String
}

fn main() {
    let mut f = File::open("rfc2812_replies.txt").unwrap();

    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let mut lines = content.lines();
    let mut line = lines.next();

    let mut replies = Vec::new();

    while let Some(l) = line {
        // if is new command
        if l.chars().next().map(char::is_whitespace) == Some(false) {
            let t = l.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
            assert_eq!(t.len(), 2);
            replies.push(Reply {
                number: t[0].to_owned(),
                reply: t[1].to_owned(),
                doc: String::new()
            });
        }

        let len = replies.len();
        replies[len - 1].doc.push_str(l);
        replies[len - 1].doc.push_str("\n");

        line = lines.next();
    }

    println!("use ::{{ Result, IrscError }};");
    println!("use std::str::FromStr;");
    println!("use std::borrow::ToOwned;");

    println!("#[allow(non_camel_case_types)]");
    println!("#[derive(Debug, Hash, PartialEq, Eq)]");
    println!("pub enum Reply {{");
    for r in &replies {
        for l in r.doc.lines() {
            println!("    /// {}", l);
        }

        println!("    {} = {},\n", r.reply, r.number);
    }
    println!("}}\n\n");

    println!("impl FromStr for Reply {{");
    println!("    type Err = IrscError;");
    println!("    fn from_str(s: &str) -> Result<Reply> {{");
    println!("        use self::Reply::*;");
    println!("        match s {{");
    for r in &replies {
        println!("            \"{}\" => Ok({}),", r.number, r.reply);
    }
    println!("            _ => Err(IrscError::NotFound)");
    println!("        }}");
    println!("     }}");
    println!("}}");

    println!("impl ToString for Reply {{");
    println!("    fn to_string(&self) -> String {{");
    println!("        use self::Reply::*;");
    println!("        match *self {{");
    for r in &replies {
        println!("            {} => \"{}\".to_owned(),", r.reply, r.number);
    }
    println!("        }}");
    println!("     }}");
    println!("}}");
}
