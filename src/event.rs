use command;
use reply;

pub enum Event<'a> {
    Command(command::Command<'a>),
    Reply(reply::Reply<'a>),
    Connected,
    Disconnected
}
