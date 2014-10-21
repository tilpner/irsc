use std::io::{
    BufferedReader,
    TcpStream,
    IoError
};
use std::ascii::StrAsciiExt;
use std::comm;
use std::comm::{ Sender, Receiver };
use std::collections::HashMap;

use std::str::UnicodeStrSlice;

use events::*;

use ident::Ident;

fn parse_msg(v: Vec<&str>, from: uint) -> String {
    let mut msg = if v[from].chars().next().unwrap() == ':' {
        v[from][1..].into_string()
    } else { v[from].into_string() };
    for m in v.iter().skip(from + 1) {
        msg.push_str(" ");
        msg.push_str(m.trim_right());
    }
    msg
}

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Failure {
    NotConnected,
    Io(IoError)
}

pub struct Context<'a> {
    prefix: &'a str,
    command: &'a str,
    parts: [&'a str]
}

pub struct Server<'a> {
    host: String,
    port: u16,
    stream: Option<TcpStream>,
    event_sender: Option<Sender<Event>>,
    event_types: HashMap<String, &'a Fn<Context<'a>, Event> + 'a>
}

impl<'a> Server<'a> {
    pub fn new(host: String, port: u16) -> Server<'a> {
        Server {
            host: host,
            port: port,
            stream: None,
            event_sender: None,
            event_types: HashMap::new()
        }
    }

    pub fn events(&mut self) -> Receiver<Event> {
        let (tx, rx) = comm::channel();
        self.events = Some(tx);
        rx
    }

    fn fire_event(&mut self, event: Event) {
        self.events.as_ref().map(|s| s.send(event.clone()));
    }

    pub fn connect(&mut self) -> Result<(), Failure> {
        self.stream = match TcpStream::connect(self.host.as_slice(), self.port) {
            Ok(tcp) => Some(tcp),
            Err(e) => return Err(Io(e))
        };

        let mut s = self.clone();
        spawn(proc() {
            s.listen();
        });
        Ok(())
    }

    #[inline]
    fn sendraw(&mut self, s: &str, newline: bool) -> Result<(), Failure> {
        println!("{}", s);
        if self.stream.is_some() {
            let mut st = self.stream.clone().unwrap();
            match st.write_str(s) {
                Ok(_) => match st.flush() {
                    Ok(_) if newline => match st.write_str("\r\n") {
                        Ok(_) => Ok(()),
                        Err(e) => return Err(Io(e))
                    },
                    Ok(_) => Ok(()),
                    Err(e) => return Err(Io(e))
                },
                Err(e) => return Err(Io(e))
            }
        } else {
            Err(NotConnected)
        }
    }

    pub fn join(&mut self, channel: &str) -> Result<(), Failure> {
        self.sendraw(format!("JOIN {}", channel).as_slice(), true)
    }

    pub fn nick(&mut self, nick: &str) -> Result<(), Failure> {
        self.sendraw(format!("NICK {}", nick).as_slice(), true)
    }

    pub fn user(&mut self, username: &str, hostname: &str, servername: &str, realname: &str) -> Result<(), Failure> {
        self.sendraw(format!("USER {} {} {} :{}", username, hostname, servername, realname).as_slice(), true)
    }

    pub fn password(&mut self, password: &str) -> Result<(), Failure> {
        self.sendraw(format!("PASS {}", password).as_slice(), true)
    }

    pub fn msg(&mut self, target: &str, message: &str) -> Result<(), Failure> {
        self.sendraw(format!("PRIVMSG {} :{}", target, message).as_slice(), true)
    }

    fn listen(&mut self) {
        let stream = match self.stream {
            Some(ref s) => s.clone(),
            None => return
        };
        let mut reader = BufferedReader::new(stream);
        loop {
            let line = reader.read_line().unwrap();
            let mut parts = line.as_slice().split(' ').collect::<Vec<&str>>();
            println!("{}", parts);
            if parts.len() == 0 {
                continue;
            }

            // if message has a prefix
            let prefix = if parts[0].chars().next().unwrap() == ':' {
                parts.remove(0).unwrap()
            } else { "" };

            let cmd = parts.remove(0).unwrap();
            let context = Context { prefix: prefix, cmd: cmd, parts: parts };
            self.events.entry(cmd).call(&context);

            /*match parts[0].to_ascii_upper().as_slice() {
                "001" => {
                    self.fire_event(RplWelcome(box Welcome {
                        source: prefix.into_string(),
                        target: parts[1].into_string(),
                        msg: parse_msg(parts, 2)
                    }))
                },
                "PING" => {
                    let _ = self.sendraw(format!("PONG {}", parts.get(1)).as_slice(), true);
                    continue;
                }
                "PRIVMSG" => {
                    let from = Ident::parse(prefix).unwrap();
                    let to = parts[1];
                    let msg = parse_msg(parts, 2);
                    self.fire_event(PrivMsg(from, to.into_string(), msg))
                },
                _ => ()
            }*/
        }
    }
}
