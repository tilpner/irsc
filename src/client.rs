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

use carboxyl::{ Stream, Sink };

use message::Message;
use command::Command;
use command::Command::*;
use reply::Reply;
use event::Event;
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

pub trait Client {
    fn send_message(&mut self, msg: Message) -> Result<()>;
    fn join(&mut self, channel: &str, password: Option<&str>) -> Result<()> {
        self.send_message(JOIN(vec![channel.into()], password.iter().map(|&p| p.into()).collect()).to_message())
    }

    fn msg(&mut self, to: &str, message: &str) -> Result<()> {
        self.send_message(PRIVMSG(to.into(), message.into()).to_message())
    }

    fn register(&mut self, nick: &str, user: &str, desc: &str, pass: Option<&str>) -> Result<()> {
        Result(if let Some(pass) = pass {
            self.send_message(PASS(pass.into()).to_message()).inner()
        } else { Ok(()) }
            .and_then(|_| self.send_message(NICK(nick.into()).to_message()).inner())
            .and_then(|_| self.send_message(USER(user.into(), Borrowed("0"), Borrowed("*"), desc.into()).to_message()).inner())
        )
    }

}

pub struct OwnedClient {
    stream: Option<StreamKind>,
    sink: Sink<Message>
}

impl OwnedClient {
    pub fn new() -> OwnedClient {
        OwnedClient {
            stream: None,
            sink: Sink::new()
        }
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
    fn send_raw(&mut self, s: &str) -> Result<()> {
        info!(">> {}", s);
        if DEBUG && s.len() > 512 {
            panic!("Message too long, kittens will die if this runs in release mode. Msg: {}", s)
        }

        Result(self.stream.as_mut()
            .ok_or(IrscError::NotConnected)
            .and_then(|mut stream| stream.write_all(s.as_bytes())
                                         .and_then(|_| stream.flush())
                                         .map_err(IrscError::Io)))
    }


    pub fn send(&mut self, cmd: Command) -> Result<()> {
        self.send_message(cmd.to_message())
    }

    pub fn listen_with_callback<F>(&mut self, on_event: F) -> Result<()>
    where F: Fn(&mut Client, &Message, Option<Event>) {
        let reader = BufReader::new(match self.stream {
            Some(StreamKind::Plain(ref s)) => StreamKind::Plain((*s).try_clone().unwrap()),
            Some(StreamKind::Ssl(ref s)) => StreamKind::Ssl((*s).try_clone().unwrap()),
            None => return Result(Err(IrscError::NotConnected))
        });

        for raw_line in reader.lines() {
            let line = raw_line.as_ref().unwrap().parse();
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

    #[allow(mutable_transmutes)]
    fn listen_with_events(&self) -> Result<()> {
        let mut s: &mut OwnedClient = unsafe { mem::transmute(self) };
        let reader = BufReader::new(match self.stream {
            Some(StreamKind::Plain(ref s)) => StreamKind::Plain((*s).try_clone().unwrap()),
            Some(StreamKind::Ssl(ref s)) => StreamKind::Ssl((*s).try_clone().unwrap()),
            None => return Result(Err(IrscError::NotConnected))
        });

        for raw_line in reader.lines() {
            let line = raw_line.as_ref().unwrap().parse();
            info!("<< {}", raw_line.unwrap());

            if let Ok(msg) = line {
                s.handle_event(&msg);
                self.sink.send(msg);
            }
        }
        Result(Ok(()))
    }

    pub fn into_shared(self) -> SharedClient {
        SharedClient {
            client: Arc::new(OwnedClientCell(UnsafeCell::new(self))),
        }
    }

    pub fn messages(&self) -> Stream<Message> { self.sink.stream() }
}

struct OwnedClientCell(UnsafeCell<OwnedClient>);
unsafe impl Sync for OwnedClientCell {}

impl Client for OwnedClient {
    fn send_message(&mut self, msg: Message) -> Result<()> {
        self.send_raw(&msg.to_string())
    }
}

#[derive(Clone)]
pub struct SharedClient {
    client: Arc<OwnedClientCell>,
}

impl SharedClient {
    pub fn messages(&self) -> Stream<(SharedClient, Message)> {
        let cl = SharedClient { client: self.client.clone() };
        unsafe { &*self.client.0.get() }.messages()
            .map(move |m| (cl.clone(), m))
    }

    pub fn events(&self) -> Stream<(SharedClient, Message, Event<'static>)> {
        self.messages().filter_map(|(cl, msg)| match Command::from_message(&msg) {
            Some(m) => Some((cl, msg.clone(), Event::Command(m.clone()).to_static())),
            None => match Reply::from_message(&msg) {
                Some(r) => Some((cl, msg.clone(), Event::Reply(r).to_static())),
                None => None
            }
        })
    }

    pub fn listen_with_events(&mut self) -> Result<()> {
        unsafe { &*self.client.0.get() }.listen_with_events()
    }

    pub fn commands(&self) -> Stream<(SharedClient, Message, Command<'static>)> {
        self.messages().filter_map(|(cl, msg)| match Command::from_message(&msg) {
            Some(m) => Some((cl, msg.clone(), m.to_static())),
            None => None
        })
    }

    pub fn replies(&self) -> Stream<(SharedClient, Message, Reply<'static>)> {
        self.messages().filter_map(|(cl, msg)| match Reply::from_message(&msg) {
            Some(m) => Some((cl, msg.clone(), m.to_static())),
            None => None
        })
    }
}

impl Client for SharedClient {
    fn send_message(&mut self, msg: Message) -> Result<()> {
        unsafe { &mut *self.client.0.get() }.send_raw(&msg.to_string())
    }
}
