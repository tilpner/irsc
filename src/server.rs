use std::io::{
    self,
    Write,
    Read,
    BufRead,
    BufReader,
};

use std::net::TcpStream;

use message;
use message::{ Command, Message };

#[cfg(feature = "ssl")]
use openssl::ssl::{ SslContext, SslMethod, SslStream };

/// Yes, I don't like the name either, but it's private, so...
enum StreamKind {
    Plain(TcpStream),
    #[cfg(feature = "ssl")]
    Ssl(SslStream)
}

impl Write for StreamKind {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            StreamKind::Plain(ref mut s) => s.write(buf),
            #[cfg(feature = "ssl")]
            StreamKind::Ssl(ref mut s) => s.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            StreamKind::Plain(ref mut s) => s.flush(),
            #[cfg(feature = "ssl")]
            StreamKind::Ssl(ref mut s) => s.flush()
        }
    }
}

impl Read for StreamKind {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            StreamKind::Plain(ref mut s) => s.read(buf),
            #[cfg(feature = "ssl")]
            StreamKind::Ssl(ref mut s) => s.read(buf)
        }
    }
}

pub struct Server {
    stream: Option<StreamKind>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            stream: None
        }
    }

    fn handle_event(&mut self, msg: &message::Message) {
        if *msg.command == "PING" {
            let _ = self.send(Command::Pong { server1: msg.suffix.clone(), server2: None }.to_message());
        }
    }

    pub fn connect(&mut self, host: String, port: u16) -> ::Result<()> {
        let s = &mut self.stream;
        match *s { Some(_) => return Err(::IrscError::AlreadyConnected), _ => () };
        *s = match TcpStream::connect((host.as_ref(), port)) {
            Ok(tcp) => Some(StreamKind::Plain(tcp)),
            Err(e) => return Err(::IrscError::Io(e))
        };

        Ok(())
    }

    #[cfg(feature = "ssl")]
    pub fn connect_ssl(&mut self, host: String, port: u16) -> ::Result<()> {
        let mut s = self.stream.lock();
        match *s { Some(_) => return Err(::IrscError::AlreadyConnected), _ => () };
        let tcp_stream = match TcpStream::connect((host.as_ref(), port)) {
            Ok(tcp) => Some(tcp),
            Err(e) => return Err(::IrscError::Io(e))
        };

        let ssl = SslContext::new(SslMethod::Tlsv1);
        let ssl_stream = SslStream::new(&ssl, tcp_stream);
        *s = ssl_stream;

        Ok(())
    }

    #[inline]
    fn sendraw(&mut self, s: &str, newline: bool) -> ::Result<usize> {
        info!(">> {}", s);
        if ::DEBUG && s.len() > 510 {
            panic!("Message too long, kitties will die if this runs in release mode. Msg: {}", s)
        }
        let stream = &mut self.stream;
        if stream.is_some() {
            stream.as_mut().map(|stream| {
                match stream.write(s.as_bytes()) {
                    Ok(a) => match { if newline { stream.write(b"\r\n").map(|s| s + a) } else { Ok(a) } } {
                        Ok(b) =>  match stream.flush() {
                            Ok(_) => Ok(b),
                            Err(e) => return Err(::IrscError::Io(e))
                        },
                        Err(e) => return Err(::IrscError::Io(e))
                    },
                    Err(e) => return Err(::IrscError::Io(e))
                }
            }).unwrap()
        } else {
            Err(::IrscError::NotConnected)
        }
    }

    pub fn send(&mut self, msg: message::Message) -> ::Result<usize> {
        self.sendraw(&msg.to_string(), true)
    }

    pub fn join(&mut self, channel: &str) -> ::Result<usize> {
        self.sendraw(format!("JOIN {}", channel).as_ref(), true)
    }

    pub fn part(&mut self, channel: &str) -> ::Result<usize> {
        self.sendraw(format!("PART {}", channel).as_ref(), true)
    }

    pub fn nick(&mut self, nick: &str) -> ::Result<usize> {
        self.sendraw(format!("NICK {}", nick).as_ref(), true)
    }

    pub fn user(&mut self, username: &str, hostname: &str, servername: &str, realname: &str) -> ::Result<usize> {
        self.sendraw(format!("USER {} {} {} :{}", username, hostname, servername, realname).as_ref(), true)
    }

    pub fn password(&mut self, password: &str) -> ::Result<usize> {
        self.sendraw(format!("PASS {}", password).as_ref(), true)
    }

    pub fn msg(&mut self, target: &str, message: &str) -> ::Result<usize> {
        self.sendraw(format!("PRIVMSG {} :{}", target, message).as_ref(), true)
    }

    pub fn listen(&mut self, events: &[fn(&mut Server, &Message)]) -> ::Result<()> {
        let mut reader = BufReader::new(match self.stream {
            Some(StreamKind::Plain(ref s)) => (*s).try_clone().unwrap(),
            #[cfg(feature = "ssl")]
            Some(StreamKind::Ssl(ref s)) => (*s).try_clone().unwrap(),
            None => return Err(::IrscError::NotConnected)
        });

        for line in reader.lines() {
            let line = line.unwrap().parse();

            if let Ok(msg) = line {
                println!("{:?}", msg);
                self.handle_event(&msg);
                for e in events.iter() {
                    e(self, &msg)
                }
            }
        }
        Ok(())
    }
}
