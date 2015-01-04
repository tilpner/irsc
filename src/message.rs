#![allow(non_camel_case_types)]

use std::str::FromStr;
use std::borrow::ToOwned;

#[derive(Clone)]
pub struct Message {
    pub prefix: Option<String>,
    pub command: Command,
    pub content: Vec<String>,
    pub suffix: Option<String>
}

impl Message {
    pub fn new(prefix: Option<String>, command: Command, content: Vec<String>, suffix: Option<String>) -> Message {
        Message {
            prefix: prefix,
            command: command,
            content: content,
            suffix: suffix
        }
    }

    pub fn parse(i: &str) -> Option<Message> {
        let len = i.len();
        let mut s = i;
        let prefix = if len >= 1 && s.char_at(0) == ':' {
            s.find(' ').map(|i| {
                let p = s.slice_chars(1, i).to_owned();
                s = s[i..];
                p
            })
        } else { None };

        let command = s.find(' ').map(|i| {
            let p = s.slice_chars(0, i).to_owned();
            s = s[i..];
            p
        }).and_then(|c| c.parse());

        let mut content = Vec::with_capacity(15);
        let mut suffix = None;
        while s.len() > 0 {
            if s.char_at(0) == ':' {
                suffix = Some(s.slice_from(1).to_owned());
                break
            }
            s.find(' ').map(|i| {
                content.push(s.slice_chars(0, i).to_owned());
                s = s[i..];
            });
        }

        command.map(move |c| Message::new(prefix, c, content, suffix))
    }

    pub fn format(&self) -> String {
        let mut s = String::with_capacity(512);
        if let Some(ref p) = self.prefix {
            s.push(':');
            s.push_str(p[]);
            s.push(' ');
        }

        s.push_str(format!("{} ", self.command)[]);

        if let Some(ref p) = self.suffix {
            s.push(':');
            s.push_str(p[]);
        }

        s
    }
}

#[derive(Copy, Clone, Show, PartialEq, Eq, Hash)]
pub enum Command {
    PASS,
    NICK,
    USER,
    OPER,
    MODE,
    SERVICE,
    QUIT,
    SQUIT,
    JOIN,
    PART,
    TOPIC,
    NAMES,
    LIST,
    INVITE,
    KICK,
    PRIVMSG,
    NOTICE,
    MOTD,
    LUSERS,
    VERSION,
    STATS,
    LINKS,
    TIME,
    CONNECT,
    TRACE,
    ADMIN,
    INFO,
    SERVLIST,
    SQUERY,
    WHO,
    WHOIS,
    WHOWAS,
    KILL,
    PING,
    PONG,
    ERROR,
    AWAY,
    REHASH,
    DIE,
    RESTART,
    SUMMON,
    USERS,
    WALLOPS,
    USERHOST,
    ISON
}

impl FromStr for Command {
    fn from_str(s: &str) -> Option<Command> {
        use self::Command::*;
        match s {
            "PASS" => Some(PASS),
            "NICK" => Some(NICK),
            "USER" => Some(USER),
            "OPER" => Some(OPER),
            "MODE" => Some(MODE),
            "SERVICE" => Some(SERVICE),
            "QUIT" => Some(QUIT),
            "SQUIT" => Some(SQUIT),
            "JOIN" => Some(JOIN),
            "PART" => Some(PART),
            "TOPIC" => Some(TOPIC),
            "NAMES" => Some(NAMES),
            "LIST" => Some(LIST),
            "INVITE" => Some(INVITE),
            "KICK" => Some(KICK),
            "PRIVMSG" => Some(PRIVMSG),
            "NOTICE" => Some(NOTICE),
            "MOTD" => Some(MOTD),
            "LUSERS" => Some(LUSERS),
            "VERSION" => Some(VERSION),
            "STATS" => Some(STATS),
            "LINKS" => Some(LINKS),
            "TIME" => Some(TIME),
            "CONNECT" => Some(CONNECT),
            "TRACE" => Some(TRACE),
            "ADMIN" => Some(ADMIN),
            "INFO" => Some(INFO),
            "SERVLIST" => Some(SERVLIST),
            "SQUERY" => Some(SQUERY),
            "WHO" => Some(WHO),
            "WHOIS" => Some(WHOIS),
            "WHOWAS" => Some(WHOWAS),
            "KILL" => Some(KILL),
            "PING" => Some(PING),
            "PONG" => Some(PONG),
            "ERROR" => Some(ERROR),
            "AWAY" => Some(AWAY),
            "REHASH" => Some(REHASH),
            "DIE" => Some(DIE),
            "RESTART" => Some(RESTART),
            "SUMMON" => Some(SUMMON),
            "USERS" => Some(USERS),
            "WALLOPS" => Some(WALLOPS),
            "USERHOST" => Some(USERHOST),
            "ISON" => Some(ISON),
            _ => None
        }
    }
}

#[derive(Show, Copy, PartialEq, Eq)]
pub enum Response {
    /// "Welcome to the Internet Relay Network <nick>!<user>@<host>"
    RPL_WELCOME = 001,
    /// "Your host is <servername>, running version <ver>"
    RPL_YOURHOST = 002,
    /// "This server was created <date>"
    RPL_CREATED = 003,
    /// "<servername> <version> <available user modes> <available channel modes>"
    RPL_MYINFO = 004,
    /// "Try server <server name>, port <port number>"
    /// Sent by the server to a user to suggest an alternative
    /// server.  This is often used when the connection is
    /// refused because the server is already full.
    RPL_BOUNCE = 005,
    /// ":*1<reply> *( " " <reply> )"
    /// - Reply format used by USERHOST to list replies to
    ///   the query list.  The reply string is composed as
    ///   follows:
    ///
    ///   reply = nickname [ "*" ] "=" ( "+" / "-" ) hostname
    ///
    ///   The '*' indicates whether the client has registered
    ///   as an Operator.  The '-' or '+' characters represent
    ///   whether the client has set an AWAY message or not
    ///   respectively.
    RPL_USERHOST = 302,
    /// ":*1<nick> *( " " <nick> )"
    /// - Reply format used by ISON to list replies to the query list.
    RPL_ISON = 303,
    /// "<nick> :<away message>"
    RPL_AWAY = 301,
    /// ":You are no longer marked as being away"
    RPL_UNAWAY = 305,
    /// ":You have been marked as being away"
    /// - These replies are used with the AWAY command (if
    ///   allowed).  RPL_AWAY is sent to any client sending a
    ///   PRIVMSG to a client which is away.  RPL_AWAY is only
    ///   sent by the server to which the client is connected.
    ///   Replies RPL_UNAWAY and RPL_NOWAWAY are sent when the
    ///   client removes and sets an AWAY message./
    RPL_NOWAWAY = 306,
    /// "<nick> <user> <host> * :<real name>"
    RPL_WHOISUSER = 311,
    /// "<nick> <server> :<server info>"
    RPL_WHOISSERVER = 312,
    /// "<nick> :is an IRC operator"
    RPL_WHOISOPERATOR = 313,
    /// "<nick> <integer> :seconds idle"
    RPL_WHOISIDLE = 317,
    /// "<nick> :End of WHOIS list"
    RPL_ENDOFWHOIS = 318,
    /// "<nick> :*( ( "@" / "+" ) <channel> " " )"
    /// - Replies 311 - 313, 317 - 319 are all replies
    ///   generated in response to a WHOIS message.  Given that
    ///   there are enough parameters present, the answering
    ///   server MUST either formulate a reply out of the above
    ///   numerics (if the query nick is found) or return an
    ///   error reply.  The '*' in RPL_WHOISUSER is there as
    ///   the literal character and not as a wild card.  For
    ///   each reply set, only RPL_WHOISCHANNELS may appear
    ///   more than once (for long lists of channel names).
    ///   The '@' and '+' characters next to the channel name
    ///   indicate whether a client is a channel operator or
    ///   has been granted permission to speak on a moderated
    ///   channel.  The RPL_ENDOFWHOIS reply is used to mark
    ///   the end of processing a WHOIS message.
    RPL_WHOISCHANNELS = 319,
    /// "<nick> <user> <host> * :<real name>"
    RPL_WHOWASUSER = 314,
    /// "<nick> :End of WHOWAS"
    /// - When replying to a WHOWAS message, a server MUST use
    ///   the replies RPL_WHOWASUSER, RPL_WHOISSERVER or
    ///   ERR_WASNOSUCHNICK for each nickname in the presented
    ///   list.  At the end of all reply batches, there MUST
    ///   be RPL_ENDOFWHOWAS (even if there was only one reply
    ///   and it was an error).
    RPL_ENDOFWHOWAS = 369,
    /// Obsolete. Not used.
    #[deprecated = "Obsolete. Not used."]
    RPL_LISTSTART = 321,
    /// "<channel> <# visible> :<topic>"
    RPL_LIST = 322,
    /// ":End of LIST"
    /// - Replies RPL_LIST, RPL_LISTEND mark the actual replies
    ///   with data and end of the server's response to a LIST
    ///   command.  If there are no channels available to return,
    ///   only the end reply MUST be sent.
    RPL_LISTEND = 323,
    /// "<channel> <nickname>"
    RPL_UNIQOPIS = 325,
    /// "<channel> <mode> <mode params>"
    RPL_CHANNELMODEIS = 324,
    /// "<channel> :No topic is set"
    RPL_NOTOPIC = 331,
    /// "<channel> :<topic>"
    /// - When sending a TOPIC message to determine the
    ///   channel topic, one of two replies is sent.  If
    ///   the topic is set, RPL_TOPIC is sent back else
    ///   RPL_NOTOPIC.
    RPL_TOPIC = 332,
    /// "<channel> <nick>"
    /// - Returned by the server to indicate that the
    ///   attempted INVITE message was successful and is
    ///   being passed onto the end client.
    RPL_INVITING = 341,
    /// "<user> :Summoning user to IRC"
    /// - Returned by a server answering a SUMMON message to
    ///   indicate that it is summoning that user.
    RPL_SUMMONING = 342,
    /// "<channel> <invitemask>"
    RPL_INVITELIST = 346,
    /// "<channel> :End of channel invite list"
    /// - When listing the 'invitations masks' for a given channel,
    ///   a server is required to send the list back using the
    ///   RPL_INVITELIST and RPL_ENDOFINVITELIST messages.  A
    ///   separate RPL_INVITELIST is sent for each active mask.
    ///   After the masks have been listed (or if none present) a
    ///   RPL_ENDOFINVITELIST MUST be sent.
    RPL_ENDOFINVITELIST = 347,
    /// "<channel> <exceptionmask>"
    RPL_EXCEPTLIST = 348,
    /// "<channel> :End of channel exception list"
    /// - When listing the 'exception masks' for a given channel,
    ///   a server is required to send the list back using the
    ///   RPL_EXCEPTLIST and RPL_ENDOFEXCEPTLIST messages.  A
    ///   separate RPL_EXCEPTLIST is sent for each active mask.
    ///   After the masks have been listed (or if none present)
    ///   a RPL_ENDOFEXCEPTLIST MUST be sent./
    RPL_ENDOFEXCEPTLIST = 349,
    /// "<version>.<debuglevel> <server> :<comments>"
    ///  - Reply by the server showing its version details.
    ///    The <version> is the version of the software being
    ///    used (including any patchlevel revisions) and the
    ///    <debuglevel> is used to indicate if the server is
    ///    running in "debug mode".
    ///
    ///    The "comments" field may contain any comments about
    ///    the version or further version details.
    RPL_VERSION = 351,
    /// "<channel> <user> <host> <server> <nick>
    ///       ( "H" / "G" > ["*"] [ ( "@" / "+" ) ]
    ///       :<hopcount> <real name>"
    RPL_WHOREPLY = 352,
    /// "<name> :End of WHO list"
    /// - The RPL_WHOREPLY and RPL_ENDOFWHO pair are used
    ///    to answer a WHO message.  The RPL_WHOREPLY is only
    ///    sent if there is an appropriate match to the WHO
    ///    query.  If there is a list of parameters supplied
    ///    with a WHO message, a RPL_ENDOFWHO MUST be sent
    ///    after processing each list item with <name> being
    ///    the item.
    RPL_ENDOFWHO = 315,
    /// "( "=" / "*" / "@" ) <channel>
    ///      :[ "@" / "+" ] <nick> *( " " [ "@" / "+" ] <nick> )
    /// - "@" is used for secret channels, "*" for private
    ///   channels, and "=" for others (public channels).
    RPL_NAMREPLY = 353,
    /// "<channel> :End of NAMES list"
    /// - To reply to a NAMES message, a reply pair consisting
    ///   of RPL_NAMREPLY and RPL_ENDOFNAMES is sent by the
    ///   server back to the client.  If there is no channel
    ///   found as in the query, then only RPL_ENDOFNAMES is
    ///   returned.  The exception to this is when a NAMES
    ///   message is sent with no parameters and all visible
    ///   channels and contents are sent back in a series of
    ///   RPL_NAMEREPLY messages with a RPL_ENDOFNAMES to mark
    ///   the end.
    RPL_ENDOFNAMES = 366,
    /// "<mask> <server> :<hopcount> <server info>"
    RPL_LINKS = 364,
    /// "<mask> :End of LINKS list"
    /// - In replying to the LINKS message, a server MUST send
    ///   replies back using the RPL_LINKS numeric and mark the
    ///   end of the list using an RPL_ENDOFLINKS reply.
    RPL_ENDOFLINKS = 365,
    /// "<channel> <banmask>"
    RPL_BANLIST = 367,
    /// "<channel> :End of channel ban list"
    /// - When listing the active 'bans' for a given channel,
    ///   a server is required to send the list back using the
    ///   RPL_BANLIST and RPL_ENDOFBANLIST messages.  A separate
    ///   RPL_BANLIST is sent for each active banmask.  After the
    ///   banmasks have been listed (or if none present) a
    ///   RPL_ENDOFBANLIST MUST be sent.
    RPL_ENDOFBANLIST = 368,
    /// ":<string>"
    RPL_INFO = 371,
    /// ":End of INFO list"
    /// - A server responding to an INFO message is required to
    ///   send all its 'info' in a series of RPL_INFO messages
    ///   with a RPL_ENDOFINFO reply to indicate the end of the
    ///   replies.
    RPL_ENDOFINFO = 374,
    /// ":- <server> Message of the day - "
    RPL_MOTDSTART = 375,
    /// ":- <text>"
    RPL_MOTD = 372,
    /// ":End of MOTD command"
    /// - When responding to the MOTD message and the MOTD file
    ///   is found, the file is displayed line by line, with
    ///   each line no longer than 80 characters, using
    ///   RPL_MOTD format replies.  These MUST be surrounded
    ///   by a RPL_MOTDSTART (before the RPL_MOTDs) and an
    ///   RPL_ENDOFMOTD (after).
    RPL_ENDOFMOTD = 376,
    /// ":You are now an IRC operator"
    /// - RPL_YOUREOPER is sent back to a client which has
    ///   just successfully issued an OPER message and gained
    ///   operator status.
    RPL_YOUREOPER = 381,
    /// "<config file> :Rehashing"
    /// - If the REHASH option is used and an operator sends
    ///   a REHASH message, an RPL_REHASHING is sent back to
    ///   the operator.
    RPL_REHASHING = 382,
    /// "You are service <servicename>"
    /// - Sent by the server to a service upon successful
    ///   registration.
    RPL_YOURESERVICE = 383,
    /// "<server> :<string showing server's local time>"
    /// - When replying to the TIME message, a server MUST send
    ///   the reply using the RPL_TIME format above.  The string
    ///   showing the time need only contain the correct day and
    ///   time there.  There is no further requirement for the
    ///   time string.
    RPL_TIME = 391,
    /// ":UserID   Terminal  Host"
    RPL_USERSSTART = 392,
    /// ":<username> <ttyline> <hostname>"
    RPL_USERS = 393,
    /// ":End of users"
    RPL_ENDOFUSERS = 394,
    /// ":Nobody logged in"
    /// - If the USERS message is handled by a server, the
    ///   replies RPL_USERSTART, RPL_USERS, RPL_ENDOFUSERS and
    ///   RPL_NOUSERS are used.  RPL_USERSSTART MUST be sent
    ///   first, following by either a sequence of RPL_USERS
    ///   or a single RPL_NOUSER.  Following this is
    ///   RPL_ENDOFUSERS.
    RPL_NOUSERS = 395,
    /// "Link <version & debug level> <destination>
    ///       <next server> V<protocol version>
    ///       <link uptime in seconds> <backstream sendq>
    ///       <upstream sendq>"
    RPL_TRACELINK = 200,
    /// "Try. <class> <server>"
    RPL_TRACECONNECTING = 201,
    /// "H.S. <class> <server>"
    RPL_TRACEHANDSHAKE = 202,
    /// "???? <class> [<client IP address in dot form>]"
    RPL_TRACEUNKNOWN = 203,
    /// "Oper <class> <nick>"
    RPL_TRACEOPERATOR = 204,
    /// "User <class> <nick>"
    RPL_TRACEUSER = 205,
    /// "Serv <class> <int>S <int>C <server>
    ///       <nick!user|*!*>@<host|server> V<protocol version>"
    RPL_TRACESERVER = 206,
    /// "Service <class> <name> <type> <active type>"
    RPL_TRACESERVICE = 207,
    /// "<newtype> 0 <client name>"
    RPL_TRACENEWTYPE = 208,
    /// "Class <class> <count>"
    RPL_TRACECLASS = 209,
    /// Unused.
    #[deprecated = "Unused."]
    RPL_TRACERECONNECT = 210,
    /// "File <logfile> <debug level>"
    RPL_TRACELOG = 261,
    /// "<server name> <version & debug level> :End of TRACE"
    /// - The RPL_TRACE* are all returned by the server in
    ///   response to the TRACE message.  How many are
    ///   returned is dependent on the TRACE message and
    ///   whether it was sent by an operator or not.  There
    ///   is no predefined order for which occurs first.
    ///   Replies RPL_TRACEUNKNOWN, RPL_TRACECONNECTING and
    ///   RPL_TRACEHANDSHAKE are all used for connections
    ///   which have not been fully established and are either
    ///   unknown, still attempting to connect or in the
    ///   process of completing the 'server handshake'.
    ///   RPL_TRACELINK is sent by any server which handles
    ///   a TRACE message and has to pass it on to another
    ///   server.  The list of RPL_TRACELINKs sent in
    ///   response to a TRACE command traversing the IRC
    ///   network should reflect the actual connectivity of
    ///   the servers themselves along that path.
    ///   RPL_TRACENEWTYPE is to be used for any connection
    ///   which does not fit in the other categories but is
    ///   being displayed anyway.
    ///   RPL_TRACEEND is sent to indicate the end of the list./
    RPL_TRACEEND = 262,
    /// "<linkname> <sendq> <sent messages>
    ///       <sent Kbytes> <received messages>
    ///       <received Kbytes> <time open>"
    /// - reports statistics on a connection.  <linkname>
    ///   identifies the particular connection, <sendq> is
    ///   the amount of data that is queued and waiting to be
    ///   sent <sent messages> the number of messages sent,
    ///   and <sent Kbytes> the amount of data sent, in
    ///   Kbytes. <received messages> and <received Kbytes>
    ///   are the equivalent of <sent messages> and <sent
    ///   Kbytes> for received data, respectively.  <time
    ///   open> indicates how long ago the connection was
    ///   opened, in seconds.
    RPL_STATSLINKINFO = 211,
    /// "<command> <count> <byte count> <remote count>"
    /// - reports statistics on commands usage.
    RPL_STATSCOMMAND = 212,
    /// "<stats letter> :End of STATS report"
    RPL_ENDOFSTATS = 219,
    /// ":Server Up %d days %d:%02d:%02d"
    /// - reports the server uptime.
    RPL_STATSUPTIME = 242,
    /// "O <hostmask> * <name>"
    /// - reports the allowed hosts from where user may become IRC
    ///   operators.
    RPL_STATSOLINE = 243,
    /// "<user mode string>"
    /// - To answer a query about a client's own mode,
    ///   RPL_UMODEIS is sent back.
    RPL_UMODEIS = 221,
    /// "<name> <server> <mask> <type> <hopcount> <info>"
    RPL_SERVLIST = 234,
    /// "<mask> <type> :End of service listing"
    /// - When listing services in reply to a SERVLIST message,
    ///   a server is required to send the list back using the
    ///   RPL_SERVLIST and RPL_SERVLISTEND messages.  A separate
    ///   RPL_SERVLIST is sent for each service.  After the
    ///   services have been listed (or if none present) a
    ///   RPL_SERVLISTEND MUST be sent./
    RPL_SERVLISTEND = 235,
    /// ":There are <integer> users and <integer>
    ///      services on <integer> servers"
    RPL_LUSERCLIENT = 251,
    /// "<integer> :operator(s) online"
    RPL_LUSEROP = 252,
    /// "<integer> :unknown connection(s)"
    RPL_LUSERUNKNOWN = 253,
    /// "<integer> :channels formed"
    RPL_LUSERCHANNELS = 254,
    /// ":I have <integer> clients and <integer>
    ///       servers"
    /// - In processing an LUSERS message, the server
    ///   sends a set of replies from RPL_LUSERCLIENT,
    ///   RPL_LUSEROP, RPL_USERUNKNOWN,
    ///   RPL_LUSERCHANNELS and RPL_LUSERME.  When
    ///   replying, a server MUST send back
    ///   RPL_LUSERCLIENT and RPL_LUSERME.  The other
    ///   replies are only sent back if a non-zero count
    ///   is found for them.
    RPL_LUSERME = 255,
    /// "<server> :Administrative info"
    RPL_ADMINME = 256,
    /// ":<admin info>"
    RPL_ADMINLOC1 = 257,
    /// ":<admin info>"
    RPL_ADMINLOC2 = 258,
    /// ":<admin info>"
    /// - When replying to an ADMIN message, a server
    ///   is expected to use replies RPL_ADMINME
    ///   through to RPL_ADMINEMAIL and provide a text
    ///   message with each.  For RPL_ADMINLOC1 a
    ///   description of what city, state and country
    ///   the server is in is expected, followed by
    ///   details of the institution (RPL_ADMINLOC2)
    ///   and finally the administrative contact for the
    ///   server (an email address here is REQUIRED)
    ///   in RPL_ADMINEMAIL.
    RPL_ADMINEMAIL = 259,
    /// "<command> :Please wait a while and try again."
    /// - When a server drops a command without processing it,
    ///   it MUST use the reply RPL_TRYAGAIN to inform the
    ///   originating client.
    RPL_TRYAGAIN = 263,

    /// "<nickname> :No such nick/channel"
    /// - Used to indicate the nickname parameter supplied to a
    ///   command is currently unused.
    ERR_NOSUCHNICK = 401,
    /// "<server name> :No such server"
    /// - Used to indicate the server name given currently
    ///   does not exist.
    ERR_NOSUCHSERVER = 402,
    /// "<channel name> :No such channel"
    /// - Used to indicate the given channel name is invalid.
    ERR_NOSUCHCHANNEL = 403,
    /// "<channel name> :Cannot send to channel"
    /// - Sent to a user who is either (a) not on a channel
    ///   which is mode +n or (b) not a chanop (or mode +v) on
    ///   a channel which has mode +m set or where the user is
    ///   banned and is trying to send a PRIVMSG message to
    ///   that channel.
    ERR_CANNOTSENDTOCHAN = 404,
    /// "<channel name> :You have joined too many channels"
    /// - Sent to a user when they have joined the maximum
    ///   number of allowed channels and they try to join
    ///   another channel.
    ERR_TOOMANYCHANNELS = 405,
    /// "<nickname> :There was no such nickname"
    /// - Returned by WHOWAS to indicate there is no history
    ///   information for that nickname.
    ERR_WASNOSUCHNICK = 406,
    /// "<target> :<error code> recipients. <abort message>"
    /// - Returned to a client which is attempting to send a
    ///   PRIVMSG/NOTICE using the user@host destination format
    ///   and for a user@host which has several occurrences.
    /// - Returned to a client which trying to send a
    ///   PRIVMSG/NOTICE to too many recipients.
    /// - Returned to a client which is attempting to JOIN a safe
    ///   channel using the shortname when there are more than one
    ///   such channel.
    ERR_TOOMANYTARGETS = 407,
    /// "<service name> :No such service"
    /// - Returned to a client which is attempting to send a SQUERY
    ///   to a service which does not exist.
    ERR_NOSUCHSERVICE = 408,
    /// ":No origin specified"
    /// - PING or PONG message missing the originator parameter.
    ERR_NOORIGIN = 409,
    /// ":No recipient given (<command>)"
    ERR_NORECIPIENT = 411,
    /// ":No text to send"
    ERR_NOTEXTTOSEND = 412,
    /// "<mask> :No toplevel domain specified"
    ERR_NOTOPLEVEL = 413,
    /// "<mask> :Wildcard in toplevel domain"
    ERR_WILDTOPLEVEL = 414,
    /// "<mask> :Bad Server/host mask"
    /// - 412 - 415 are returned by PRIVMSG to indicate that
    ///   the message wasn't delivered for some reason.
    ///   ERR_NOTOPLEVEL and ERR_WILDTOPLEVEL are errors that
    ///   are returned when an invalid use of
    ///   "PRIVMSG $<server>" or "PRIVMSG #<host>" is attempted.
    ERR_BADMASK = 415,
    /// "<command> :Unknown command"
    /// - Returned to a registered client to indicate that the
    ///   command sent is unknown by the server.
    ERR_UNKNOWNCOMMAND = 421,
    /// ":MOTD File is missing"
    /// - Server's MOTD file could not be opened by the server.
    ERR_NOMOTD = 422,
    /// "<server> :No administrative info available"
    /// - Returned by a server in response to an ADMIN message
    ///   when there is an error in finding the appropriate
    ///   information.
    ERR_NOADMININFO = 423,
    /// ":File error doing <file op> on <file>"
    /// - Generic error message used to report a failed file
    ///   operation during the processing of a message.
    ERR_FILEERROR = 424,
    /// ":No nickname given"
    /// - Returned when a nickname parameter expected for a
    ///   command and isn't found.
    ERR_NONICKNAMEGIVEN = 431,
    /// "<nick> :Erroneous nickname"
    /// - Returned after receiving a NICK message which contains
    ///   characters which do not fall in the defined set.  See
    ///   section 2.3.1 for details on valid nicknames.
    ERR_ERRONEUSNICKNAME = 432,
    /// "<nick> :Nickname is already in use"
    /// - Returned when a NICK message is processed that results
    ///   in an attempt to change to a currently existing
    ///   nickname.
    ERR_NICKNAMEINUSE = 433,
    /// "<nick> :Nickname collision KILL from <user>@<host>"
    /// - Returned by a server to a client when it detects a
    ///   nickname collision (registered of a NICK that
    ///   already exists by another server).
    ERR_NICKCOLLISION = 436,
    /// "<nick/channel> :Nick/channel is temporarily unavailable"
    /// - Returned by a server to a user trying to join a channel
    ///   currently blocked by the channel delay mechanism.
    /// - Returned by a server to a user trying to change nickname
    ///   when the desired nickname is blocked by the nick delay
    ///   mechanism.
    ERR_UNAVAILRESOURCE = 437,
    /// "<nick> <channel> :They aren't on that channel"
    /// - Returned by the server to indicate that the target
    ///   user of the command is not on the given channel.
    ERR_USERNOTINCHANNEL = 441,
    /// "<channel> :You're not on that channel"
    /// - Returned by the server whenever a client tries to
    ///   perform a channel affecting command for which the
    ///   client isn't a member.
    ERR_NOTONCHANNEL = 442,
    /// "<user> <channel> :is already on channel"
    /// - Returned when a client tries to invite a user to a
    ///   channel they are already on.
    ERR_USERONCHANNEL = 443,
    /// "<user> :User not logged in"
    /// - Returned by the summon after a SUMMON command for a
    ///   user was unable to be performed since they were not
    ///   logged in.
    ERR_NOLOGIN = 444,
    /// ":SUMMON has been disabled"
    /// - Returned as a response to the SUMMON command.  MUST be
    ///   returned by any server which doesn't implement it.
    ERR_SUMMONDISABLED = 445,
    /// ":USERS has been disabled"
    /// - Returned as a response to the USERS command.  MUST be
    ///   returned by any server which does not implement it.
    ERR_USERSDISABLED = 446,
    /// ":You have not registered"
    /// - Returned by the server to indicate that the client
    ///   MUST be registered before the server will allow it
    ///   to be parsed in detail.
    ERR_NOTREGISTERED = 451,
    /// "<command> :Not enough parameters"
    /// - Returned by the server by numerous commands to
    ///   indicate to the client that it didn't supply enough
    ///   parameters.
    ERR_NEEDMOREPARAMS = 461,
    /// ":Unauthorized command (already registered)"
    /// - Returned by the server to any link which tries to
    ///   change part of the registered details (such as
    ///   password or user details from second USER message).
    ERR_ALREADYREGISTERED = 462,
    /// ":Your host isn't among the privileged"
    /// - Returned to a client which attempts to register with
    ///   a server which does not been setup to allow
    ///   connections from the host the attempted connection
    ///   is tried.
    ERR_NOPERMFORHOST = 463,
    /// ":Password incorrect"
    /// - Returned to indicate a failed attempt at registering
    ///   a connection for which a password was required and
    ///   was either not given or incorrect.
    ERR_PASSWDMISMATCH = 464,
    /// ":You are banned from this server"
    /// - Returned after an attempt to connect and register
    ///   yourself with a server which has been setup to
    ///   explicitly deny connections to you.
    ERR_YOUREBANNEDCREEP = 465,
    /// - Sent by a server to a user to inform that access to the
    ///   server will soon be denied.
    ERR_YOUWILLBEBANNED = 466,
    /// "<channel> :Channel key already set"
    ERR_KEYSET = 467,
    /// "<channel> :Cannot join channel (+l)"
    ERR_CHANNELISFULL = 471,
    /// "<char> :is unknown mode char to me for <channel>"
    ERR_UNKNOWNMODE = 472,
    /// "<channel> :Cannot join channel (+i)"
    ERR_INVITEONLYCHAN = 473,
    /// "<channel> :Cannot join channel (+b)"
    ERR_BANNEDFROMCHAN = 474,
    /// "<channel> :Cannot join channel (+k)"
    ERR_BADCHANNELKEY = 475,
    /// "<channel> :Bad Channel Mask"
    ERR_BADCHANMASK = 476,
    /// "<channel> :Channel doesn't support modes"
    ERR_NOCHANMODES = 477,
    /// "<channel> <char> :Channel list is full"
    ERR_BANLISTFULL = 478,
    /// ":Permission Denied- You're not an IRC operator"
    /// - Any command requiring operator privileges to operate
    ///   MUST return this error to indicate the attempt was
    ///   unsuccessful.
    ERR_NOPRIVILEGES = 481,
    /// "<channel> :You're not channel operator"
    /// - Any command requiring 'chanop' privileges (such as
    ///   MODE messages) MUST return this error if the client
    ///   making the attempt is not a chanop on the specified
    ///   channel.
    ERR_CHANOPRIVSNEEDED = 482,
    /// ":You can't kill a server!"
    /// - Any attempts to use the KILL command on a server
    ///   are to be refused and this error returned directly
    ///   to the client.
    ERR_CANTKILLSERVER = 483,
    /// ":Your connection is restricted!"
    /// - Sent by the server to a user upon connection to indicate
    ///   the restricted nature of the connection (user mode "+r").
    ERR_RESTRICTED = 484,
    /// ":You're not the original channel operator"
    /// - Any MODE requiring "channel creator" privileges MUST
    ///   return this error if the client making the attempt is not
    ///   a chanop on the specified channel.
    ERR_UNIQOPPRIVSNEEDED = 485,
    /// ":No O-lines for your host"
    /// - If a client sends an OPER message and the server has
    ///   not been configured to allow connections from the
    ///   client's host as an operator, this error MUST be
    ///   returned.
    ERR_NOOPERHOST = 491,
    /// ":Unknown MODE flag"
    /// - Returned by the server to indicate that a MODE
    ///   message was sent with a nickname parameter and that
    ///   the a mode flag sent was not recognized.
    ERR_UMODEUNKNOWNFLAG = 501,
    /// ":Cannot change mode for other users"
    /// - Error sent to any user trying to view or change the
    ///   user mode for a user other than themselves.
    ERR_USERSDONTMATCH = 502,
}

impl Response {
    pub fn is_reply(&self) -> bool { let i = *self as uint; i >= 200 && i <= 399 }
    pub fn is_error(&self) -> bool { let i = *self as uint; i >= 400 && i <= 599 }
}

pub fn join(v: Vec<String>, from: uint) -> String {
    let mut msg = if v[from].chars().next().unwrap() == ':' {
        v[from][][1..].to_owned()
    } else { v[from].clone() };
    for m in v.iter().skip(from + 1) {
        msg.push_str(" ");
        msg.push_str(m.trim_right());
    }
    msg
}

/*pub struct PrivMsg {
    pub from: Ident,
    pub to: String,
    pub content: String
}

impl ParseResult for PrivMsg {
    fn parse(message: Message) -> Option<PrivMsg> {
        let from = Ident::parse(message.prefix.unwrap()[]);
        let to = message.content[0].clone();
        match from {
            Some(from) => Some(PrivMsg {
                from: from,
                to: to,
                content: join(message.content, 1)
            }),
            None => None
        }
    }
}*/
