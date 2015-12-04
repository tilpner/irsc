use std::io::{
    self,
    Write,
    Read,
    BufRead,
    BufReader,
};
use std::net::TcpStream;
use std::borrow::Cow::{ self, Borrowed, Owned };
use std::sync::{ Arc, RwLock };
use std::mem;
use std::cell::UnsafeCell;

use message::Message;
use command::Command;
use command::Command::*;
use reply::Reply;
use event::Event;
use text::*;
use ::{ DEBUG, Result, IrscError };

use openssl::ssl::{ Ssl, SslContext, SslMethod, SslStream };

/// Yes, I don't like the name either, but it's private, so...
enum StreamKind {
    Plain(TcpStream),
    Ssl(SslStream<TcpStream>)
}

impl Write for StreamKind {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            StreamKind::Plain(ref mut s) => s.write(buf),
            StreamKind::Ssl(ref mut s) => s.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            StreamKind::Plain(ref mut s) => s.flush(),
            StreamKind::Ssl(ref mut s) => s.flush()
        }
    }
}

impl Read for StreamKind {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            StreamKind::Plain(ref mut s) => s.read(buf),
            StreamKind::Ssl(ref mut s) => s.read(buf)
        }
    }
}


pub struct Client {
    stream: Option<StreamKind>,
}

impl Client {
    pub fn new() -> Client {
        Client { stream: None }
    }

    fn handle_event(&mut self, msg: &Message) {
        let _ = match Command::from_message(msg) {
            Some(PING(s1, s2)) => self.send(PONG(s1, s2)),
            _ => Result(Ok(()))
        };
    }

    pub fn connect(&mut self, host: &str, port: u16) -> Result<()> {
        let s = &mut self.stream;
        if s.is_some() { return Result(Err(IrscError::AlreadyConnected)) }
        *s = match TcpStream::connect((host, port)) {
            Ok(tcp) => Some(StreamKind::Plain(tcp)),
            Err(e) => return Result(Err(IrscError::Io(e)))
        };

        Result(Ok(()))
    }

    pub fn connect_ssl(&mut self, host: &str, port: u16, ssl: Ssl) -> Result<()> {
        let s = &mut self.stream;
        if s.is_some() { return Result(Err(IrscError::AlreadyConnected)) };
        let tcp_stream = match TcpStream::connect((host, port)) {
            Ok(tcp) => Some(tcp),
            Err(e) => return Result(Err(IrscError::Io(e)))
        };

        match tcp_stream.map(|tcp| SslStream::new_from(ssl, tcp)) {
            Some(Ok(ssl_stream)) => {
                *s = Some(StreamKind::Ssl(ssl_stream));
                Result(Ok(()))
            },
            Some(Err(ssl_error)) => Result(Err(IrscError::Ssl(ssl_error))),
            None => Result(Err(IrscError::NotConnected))
        }
    }

    #[inline]
    fn send_raw(&mut self, s: &[u8]) -> Result<()> {
        if DEBUG && s.len() > 1024 {
            panic!("Message too long, kittens will die if this runs in release mode.")
        }

        Result(self.stream.as_mut()
            .ok_or(IrscError::NotConnected)
            .and_then(|mut stream| stream.write_all(s)
                                         .and_then(|_| stream.flush())
                                         .map_err(IrscError::Io)))
    }

    fn send_message(&mut self, msg: Message) -> Result<()> {
        self.send_raw(msg.bytes())
    }

    pub fn send(&mut self, cmd: Command) -> Result<()> {
        self.send_message(cmd.to_message())
    }

    pub fn listen<F>(&mut self, on_event: F) -> Result<()>
    where F: Fn(&mut Client, &Message, Option<Event>) {
        let reader = BufReader::new(match self.stream {
            Some(StreamKind::Plain(ref s)) => StreamKind::Plain((*s).try_clone().unwrap()),
            Some(StreamKind::Ssl(ref s)) => StreamKind::Ssl((*s).try_clone().unwrap()),
            None => return Result(Err(IrscError::NotConnected))
        });

        for raw_line in reader.lines() {
            let line = Message::parse(raw_line.as_ref().unwrap().as_bytes());
            info!("<< {}", raw_line.unwrap());

            if let Ok(msg) = line {
                self.handle_event(&msg);

                // Try to parse the message into a Command or a Reply, and call back.
                let event = match Command::from_message(&msg) {
                    Some(m) => Some(Event::Command(m)),
                    None => match Reply::from_message(&msg) {
                        Some(r) => Some(Event::Reply(r)),
                        None => None
                    }
                };
                on_event(self, &msg, event);
            }
        }
        Result(Ok(()))
    }

    fn join(&mut self, channel: &str, password: Option<&str>) -> Result<()> {
        self.send_message(JOIN(vec![channel.into()], password.iter().map(|&p| p.into()).collect()).to_message())
    }

    fn msg(&mut self, to: &str, message: &str) -> Result<()> {
        self.send_message(PRIVMSG(to.into(), message.into()).to_message())
    }

    fn msg_many(&mut self, to: &str, message: &[&str]) -> Result<()> {
        for m in message {
            self.msg(to, m);
        }
        Result(Ok(()))
    }

    fn msg_word_wrap(&mut self, to: &str, message: &str, limit: u16) -> Result<()> {
        let mut line = String::new();
        for word in message.split_whitespace() {
            if line.len() + word.len() < limit as usize {
                line.push_str(" ");
                line.push_str(word);
            } else {
                debug!("Sending {}", line);
                self.msg(to, &line);
                line.clear();
            }
        }
        self.msg(to, &line)
    }

    fn register(&mut self, nick: &str, user: &str, desc: &str, pass: Option<&str>) -> Result<()> {
        Result(if let Some(pass) = pass {
            self.send_message(PASS(pass.into()).to_message()).inner()
        } else { Ok(()) }
            .and_then(|_| self.send_message(NICK(nick.into()).to_message()).inner())
            .and_then(|_| self.send_message(USER(user.into(), tsu("0"), tsu("*"), desc.into()).to_message()).inner())
        )
    }
}
