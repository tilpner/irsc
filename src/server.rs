use std::io::{
    BufferedReader,
    TcpStream,
    IoError,
    IoResult
};
use std::mem;

use callback::Callback;
use message;
use message::{ Command, Message };

#[cfg(feature = "ssl")]
use openssl::ssl::{ SslContext, SslMethod, SslStream };

#[derive(Show, PartialEq, Eq, Clone)]
pub enum Failure {
    AlreadyConnected,
    NotConnected,
    Io(IoError)
}

/// Yes, I don't like the name either, but it's private, so...
enum StreamKind {
    Plain(TcpStream),
    #[cfg(feature = "ssl")]
    Ssl(SslStream)
}

impl Writer for StreamKind {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        match *self {
            StreamKind::Plain(ref mut s) => s.write(buf),
            #[cfg(feature = "ssl")]
            StreamKind::Ssl(ref mut s) => s.write(buf)
        }
    }
}

impl Reader for StreamKind {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        match *self {
            StreamKind::Plain(ref mut s) => s.read(buf),
            #[cfg(feature = "ssl")]
            StreamKind::Ssl(ref mut s) => s.read(buf)
        }
    }
}

pub struct Event<'a> {
    pub server: &'a mut Server<'a>,
    pub message: &'a Message
}

pub struct Server<'a> {
    stream: Option<StreamKind>,
    pub events: Callback<Event<'a>>,
}

impl<'a> Server<'a> {
    pub fn new() -> Server<'a> {
        Server {
            stream: None,
            events: {
                let mut c = Callback::new();
                c.register(Server::handle_event);
                c
            }
        }
    }

    fn handle_event(e: &mut Event) {
        match e.message.command {
            Command::PING => {
                e.server.sendraw(format!("PONG :{}", message::join(e.message.content.clone(), 0)).as_slice(), true).unwrap();
            }
            _ => ()
        }
    }

    pub fn connect(&mut self, host: String, port: u16) -> Result<(), Failure> {
        let s = &mut self.stream;
        match *s { Some(_) => return Err(Failure::AlreadyConnected), _ => () };
        *s = match TcpStream::connect((host.as_slice(), port)) {
            Ok(tcp) => Some(StreamKind::Plain(tcp)),
            Err(e) => return Err(Failure::Io(e))
        };

        Ok(())
    }

    #[cfg(feature = "ssl")]
    pub fn connect_ssl(&mut self, host: String, port: u16) -> Result<(), Failure> {
        let mut s = self.stream.lock();
        match *s { Some(_) => return Err(Failure::AlreadyConnected), _ => () };
        let tcp_stream = match TcpStream::connect((host.as_slice(), port)) {
            Ok(tcp) => Some(tcp),
            Err(e) => return Err(Failure::Io(e))
        };

        let ssl = SslContext::new(SslMethod::Tlsv1);
        let ssl_stream = SslStream::new(&ssl, tcp_stream);
        *s = ssl_stream;

        Ok(())

    }

    #[inline]
    fn sendraw(&mut self, s: &str, newline: bool) -> Result<(), Failure> {
        info!(">> {}", s);
        if cfg!(not(ndebug)) && s.len() > 510 {
            panic!("Message too long, kitties will die if this runs in release mode. Msg: {}", s)
        }
        let stream = &mut self.stream;
        if stream.is_some() {
            stream.as_mut().map(|stream| {
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

    pub fn send(&mut self, msg: message::Message) -> Result<(), Failure> {
        self.sendraw(msg.format()[], true)
    }

    pub fn join(&mut self, channel: &str) -> Result<(), Failure> {
        self.sendraw(format!("JOIN {}", channel).as_slice(), true)
    }

    pub fn part(&mut self, channel: &str) -> Result<(), Failure> {
        self.sendraw(format!("PART {}", channel).as_slice(), true)
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
            match self.stream {
                Some(ref s) => s.clone(),
                None => return Err(Failure::NotConnected)
            }
        };

        let mut reader = BufferedReader::new(match *stream {
            StreamKind::Plain(ref s) => (*s).clone(),
            #[cfg(feature = "ssl")]
            StreamKind::Ssl(ref s) => (*s).clone()
        });

        loop {
            let line = reader.read_line().unwrap();
            info!("<< {}", line);

            if let Some(msg) = Message::parse(line[]) {
                let mut e = self.events;
                e.fire(&mut Event {
                    server: self,
                    message: &msg
                });
            }
        }
    }
}
