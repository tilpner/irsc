use std::io::{BufReader, BufRead, Write};
use std::io::Error as IoError;
use std::net::TcpStream;

use std::sync::Arc;
use std::sync::Mutex;

use std::borrow::ToOwned;

use callback::Callback;
use event;
use event::Event;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Failure {
    AlreadyConnected,
    NotConnected,
    Io(String)
}

impl From<Failure> for String {
    fn from(f: Failure)->Self {
        format!("{:?}", f)
    }
}

#[derive(Clone)]
pub struct Server {
    stream: Arc<Mutex<Option<TcpStream>>>,
    pub events: Arc<Mutex<Callback<(Server, Event)>>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            stream: Arc::new(Mutex::new(None)),
            events: {
                let mut c = Callback::new();
                c.register(&(Server::handle_event as fn((Server,Event))));
                Arc::new(Mutex::new(c))
            }
        }
    }

    fn handle_event(arg: (Server, Event)) {
        let (mut server, event) = arg;
        match &*event.command {
            event::PING => {
                server.sendraw(&format!("PONG {:?}", event.content), true).unwrap();
            }
            _ => ()
        }
    }

    pub fn connect(&mut self, host: String, port: u16) -> Result<(), Failure> {
        let s = &mut *self.stream.lock().unwrap();
        if s.is_some() { return Err(Failure::AlreadyConnected) }
        match TcpStream::connect((&*host, port)) {
            Ok(stream) => Ok(*s = Some(stream)),
            Err(err) => Err(Failure::Io(format!("{:?}", err)))
        }
    }

    #[inline]
    fn sendraw(&mut self, s: &str, newline: bool) -> Result<(), Failure> {
        fn handle(stream: &mut TcpStream, s: &str, newline: bool)->Result<(), IoError> {
            try!(stream.write(s.as_bytes()));
            if newline { try!(stream.write(b"\r\n")); }
            try!(stream.flush());
            Ok(())
        }
        info!("OUT: {}", s);
        let locked_stream = self.stream.lock();
        let mut stream = if let Ok(stream) = locked_stream {
            stream
        } else {
            return Err(Failure::NotConnected)
        };
        if let &mut Some(ref mut stream) = &mut *stream {
            if let Err(err) = handle(stream, s, newline) {
                Err(Failure::Io(format!("{:?}", err)))
            } else {
                Ok(())
            }
        } else {
            return Err(Failure::NotConnected)
        }
    }

    pub fn join(&mut self, channel: &str) -> Result<(), Failure> {
        self.sendraw(&format!("JOIN {}", channel), true)
    }

    pub fn nick(&mut self, nick: &str) -> Result<(), Failure> {
        self.sendraw(&format!("NICK {}", nick), true)
    }

    pub fn user(&mut self, username: &str, hostname: &str, servername: &str, realname: &str)
                                                                        -> Result<(), Failure> {
        self.sendraw(&format!("USER {} {} {} :{}", username, hostname, servername, realname), true)
    }

    pub fn password(&mut self, password: &str) -> Result<(), Failure> {
        self.sendraw(&format!("PASS {}", password), true)
    }

    pub fn msg(&mut self, target: &str, message: &str) -> Result<(), Failure> {
        self.sendraw(&format!("PRIVMSG {} :{}", target, message), true)
    }

    pub fn listen(&mut self) -> Result<(), Failure> {
        let lock = if let Ok(lock) = self.stream.lock() {
            lock
        } else {
            return Err(Failure::NotConnected)
        };
        let stream = {
            match &*lock {
                &Some(ref s) => s,
                &None => return Err(Failure::NotConnected)
            }
        };

        let mut reader = BufReader::new(stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let mut parts = line.split(' ').collect::<Vec<&str>>();

            info!("IN: {}", line);

            if parts.len() == 0 {
                continue;
            }

            // if message has a prefix
            let prefix = if parts[0].chars().next().unwrap() == ':' {
                parts.remove(0)
            } else {
                ""
            };

            let cmd = parts.remove(0);
            let event = Event {
                prefix: prefix.to_owned(),
                command: cmd.to_owned(),
                content: parts.into_iter().map(|s| s.to_owned()).collect()
            };

            self.events.lock().unwrap().fire(&(self.clone(), event));
        }
    }
}
