use ident::Ident;

macro_rules! string_record(
    ($name: ident, $( $fields: ident ),*) => (
        #[deriving(Show, Clone)]
        pub struct $name {
            $(pub $fields: String),*
        }
    )
)

string_record!(Welcome, source, target, msg)
string_record!(YourHost, source, target, msg)
string_record!(Created, source, target, msg)

#[deriving(Show, Clone)]
pub enum Event {
    RplWelcome(Box<Welcome>),
    RplYourHost(Box<YourHost>),
    RplCreated(Box<Created>),
    PrivMsg(Ident, String, String)
}
