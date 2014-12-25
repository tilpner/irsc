use std::io::{
    BufferedReader,
    TcpStream,
    IoError
};

use std::sync::Arc;
use std::sync::Mutex;

use callback::Callback;
use event;
use event::Event;

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Failure {
    AlreadyConnected,
    NotConnected,
    Io(IoError)
}

#[deriving(Clone)]
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
        match event.command[] {
            event::PING => {
                server.sendraw(format!("PONG {}", event.content).as_slice(), true).unwrap();
            }
            _ => ()
        }
    }

    pub fn connect(&mut self, host: String, port: u16) -> Result<(), Failure> {
        let mut s = self.stream.lock();
        match *s { Some(_) => return Err(Failure::AlreadyConnected), _ => () };
        *s = match TcpStream::connect((host.as_slice(), port)) {
            Ok(tcp) => Some(tcp),
            Err(e) => return Err(Failure::Io(e))
        };

        Ok(())
    }

    #[inline]
    fn sendraw(&mut self, s: &str, newline: bool) -> Result<(), Failure> {
        info!("OUT: {}", s);
        let mut locked_stream = self.stream.lock();
        if locked_stream.is_some() {
            locked_stream.as_mut().map(|stream| {
                match stream.write_str(s) {
                    Ok(_) => match { if newline { stream.write_str("\r\n") } else { Ok(()) } } {
                        Ok(_) =>  match stream.flush() {
                            Ok(_) => Ok(()),
                            Err(e) => return Err(Failure::Io(e))
                        },
                        Err(e) => return Err(Failure::Io(e))
                    },
                    Err(e) => return Err(Failure::Io(e))
                }
            }).unwrap()
        } else {
            Err(Failure::NotConnected)
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

    pub fn listen(&mut self) -> Result<(), Failure> {
        let stream = {
            let lock = self.stream.lock();
            match *lock {
                Some(ref s) => s.clone(),
                None => return Err(Failure::NotConnected)
            }
        };

        let mut reader = BufferedReader::new(stream);
        loop {
            let line = reader.read_line().unwrap();
            let mut parts = line.as_slice().split(' ').collect::<Vec<&str>>();

            info!("IN: {}", line);

            if parts.len() == 0 {
                continue;
            }

            // if message has a prefix
            let prefix = if parts[0].chars().next().unwrap() == ':' {
                parts.remove(0).unwrap()
            } else { "" };

            let cmd = parts.remove(0).unwrap();
            let event = Event {
                prefix: prefix.into_string(),
                command: cmd.into_string(),
                content: parts.iter().map(|p| p.into_string()).collect()
            };

            self.events.lock().fire(&(self.clone(), event));
        }
    }
}
