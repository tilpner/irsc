use std::str::FromStr;
use std::borrow::{ Cow, ToOwned };
use std::borrow::Cow::*;

use ::{ Result, IrscError };
use ::message::Message;

macro_rules! replies {
    ($( $name: ident {
        $id: expr, $doc: meta
        b $($borrowed_items: ty),*
        o $($owned_items: ty),*
        t $($to_names: ident),+ => $($to_exprs: expr),+
    }),+) => (
        #[allow(non_camel_case_types)]
        #[derive(Debug, Hash, Clone, PartialEq, Eq)]
        pub enum Reply<'a> {
            $(
                #[$doc]
                $name($($borrowed_items),*)
            ),+
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Hash, Clone, PartialEq, Eq)]
        pub enum OwnedReply {
            $(
                #[$doc]
                $name($($owned_items),*)
            ),+
        }

        impl<'a> Reply<'a> {
            pub fn to_owned(self) -> OwnedReply {
                match self {
                    $(
                        Reply::$name($($to_names),*) => OwnedReply::$name($($to_exprs),*)
                    ),+
                }
            }

            pub fn from_message(msg: &'a Message) -> Option<Reply<'a>> {
                use Reply::*;
                match msg.command() {
                    $(
                        $id => msg.last().map(|&e| $name(e))
                    ),+
                }
            }

            pub fn to_message(&self) -> Message {
                use Reply::*;
                match self {
                    $(
                        &$name(ref s) => Message::format(None, $id, Vec::new(), s)
                    ),+
                }
            }
        }
    )
}

replies! {
    RPL_WELCOME {
        "001", doc = r#"001    RPL_WELCOME
        "Welcome to the Internet Relay Network
        <nick>!<user>@<host>""#
        b &'a [u8]
        o Vec<u8>
        t s => s.into()
    },
    RPL_YOURHOST {
        "002", doc = r#"002    RPL_YOURHOST
        "Your host is <servername>, running version <ver>""#
        b &'a [u8]
        o Vec<u8>
        t s => s.into()
    },
    RPL_CREATED {
        "003", doc = r#"003    RPL_CREATED
        "This server was created <date>""#
        b &'a [u8]
        o Vec<u8>
        t s => s.into()
    },
    RPL_MYINFO {
        "004", doc = r#"004    RPL_MYINFO
        "<servername> <version> <available user modes>
         <available channel modes>"

        - The server sends Replies 001 to 004 to a user upon
          successful registration."#
        b &'a [u8], &'a [u8], &'a [u8]
        o Vec<u8>
        t s => s.into()
    }
}
    /*
    RPL_MYINFO(&'a [u8]),

    /// 005    RPL_BOUNCE
    ///       "Try server <server name>, port <port number>"
    ///
    ///  - Sent by the server to a user to suggest an alternative
    ///    server.  This is often used when the connection is
    ///    refused because the server is already full.
    ///
    RPL_BOUNCE(&'a [u8]),

    /// 302    RPL_USERHOST
    ///       ":*1<reply> *( " " <reply> )"
    ///
    ///  - Reply format used by USERHOST to list replies to
    ///    the query list.  The reply string is composed as
    ///    follows:
    ///
    ///    reply = nickname [ "*" ] "=" ( "+" / "-" ) hostname
    ///
    ///    The '*' indicates whether the client has registered
    ///    as an Operator.  The '-' or '+' characters represent
    ///    whether the client has set an AWAY message or not
    ///    respectively.
    ///
    RPL_USERHOST(&'a [u8]),

    /// 303    RPL_ISON
    ///       ":*1<nick> *( " " <nick> )"
    ///
    ///  - Reply format used by ISON to list replies to the
    ///    query list.
    ///
    RPL_ISON(&'a [u8]),

    /// 301    RPL_AWAY
    ///       "<nick> :<away message>"
    RPL_AWAY(&'a [u8]),

    /// 305    RPL_UNAWAY
    ///       ":You are no longer marked as being away"
    RPL_UNAWAY(&'a [u8]),

    /// 306    RPL_NOWAWAY
    ///       ":You have been marked as being away"
    ///
    ///  - These replies are used with the AWAY command (if
    ///    allowed).  RPL_AWAY is sent to any client sending a
    ///    PRIVMSG to a client which is away.  RPL_AWAY is only
    ///    sent by the server to which the client is connected.
    ///    Replies RPL_UNAWAY and RPL_NOWAWAY are sent when the
    ///    client removes and sets an AWAY message.
    ///
    RPL_NOWAWAY(&'a [u8]),

    /// 311    RPL_WHOISUSER
    ///       "<nick> <user> <host> * :<real name>"
    RPL_WHOISUSER(&'a [u8]),

    /// 312    RPL_WHOISSERVER
    ///       "<nick> <server> :<server info>"
    RPL_WHOISSERVER(&'a [u8]),

    /// 313    RPL_WHOISOPERATOR
    ///       "<nick> :is an IRC operator"
    RPL_WHOISOPERATOR(&'a [u8]),

    /// 317    RPL_WHOISIDLE
    ///       "<nick> <integer> :seconds idle"
    RPL_WHOISIDLE(&'a [u8]),

    /// 318    RPL_ENDOFWHOIS
    ///       "<nick> :End of WHOIS list"
    RPL_ENDOFWHOIS(&'a [u8]),

    /// 319    RPL_WHOISCHANNELS
    ///       "<nick> :*( ( "@" / "+" ) <channel> " " )"
    ///
    ///  - Replies 311 - 313, 317 - 319 are all replies
    ///    generated in response to a WHOIS message.  Given that
    ///    there are enough parameters present, the answering
    ///    server MUST either formulate a reply out of the above
    ///    numerics (if the query nick is found) or return an
    ///    error reply.  The '*' in RPL_WHOISUSER is there as
    ///    the literal character and not as a wild card.  For
    ///    each reply set, only RPL_WHOISCHANNELS may appear
    ///    more than once (for long lists of channel names).
    ///    The '@' and '+' characters next to the channel name
    ///    indicate whether a client is a channel operator or
    ///    has been granted permission to speak on a moderated
    ///    channel.  The RPL_ENDOFWHOIS reply is used to mark
    ///    the end of processing a WHOIS message.
    ///
    RPL_WHOISCHANNELS(&'a [u8]),

    /// 314    RPL_WHOWASUSER
    ///       "<nick> <user> <host> * :<real name>"
    RPL_WHOWASUSER(&'a [u8]),

    /// 369    RPL_ENDOFWHOWAS
    ///       "<nick> :End of WHOWAS"
    ///
    ///  - When replying to a WHOWAS message, a server MUST use
    ///    the replies RPL_WHOWASUSER, RPL_WHOISSERVER or
    ///    ERR_WASNOSUCHNICK for each nickname in the presented
    ///    list.  At the end of all reply batches, there MUST
    ///    be RPL_ENDOFWHOWAS (even if there was only one reply
    ///    and it was an error).
    ///
    RPL_ENDOFWHOWAS(&'a [u8]),

    /// 321    RPL_LISTSTART
    ///       Obsolete. Not used.
    ///
    RPL_LISTSTART,

    /// 322    RPL_LIST
    ///       "<channel> <# visible> :<topic>"
    RPL_LIST(&'a [u8]),

    /// 323    RPL_LISTEND
    ///       ":End of LIST"
    ///
    ///  - Replies RPL_LIST, RPL_LISTEND mark the actual replies
    ///    with data and end of the server's response to a LIST
    ///    command.  If there are no channels available to return,
    ///    only the end reply MUST be sent.
    ///
    RPL_LISTEND(&'a [u8]),

    /// 325    RPL_UNIQOPIS
    ///       "<channel> <nickname>"
    ///
    RPL_UNIQOPIS(&'a [u8]),

    /// 324    RPL_CHANNELMODEIS
    ///       "<channel> <mode> <mode params>"
    ///
    RPL_CHANNELMODEIS(&'a [u8]),

    /// 331    RPL_NOTOPIC
    ///       "<channel> :No topic is set"
    RPL_NOTOPIC(&'a [u8]),

    /// 332    RPL_TOPIC
    ///       "<channel> :<topic>"
    ///
    ///  - When sending a TOPIC message to determine the
    ///    channel topic, one of two replies is sent.  If
    ///    the topic is set, RPL_TOPIC is sent back else
    ///    RPL_NOTOPIC.
    ///
    RPL_TOPIC(&'a [u8]),

    /// 341    RPL_INVITING
    ///       "<channel> <nick>"
    ///
    ///  - Returned by the server to indicate that the
    ///    attempted INVITE message was successful and is
    ///    being passed onto the end client.
    ///
    RPL_INVITING(&'a [u8]),

    /// 342    RPL_SUMMONING
    ///       "<user> :Summoning user to IRC"
    ///
    ///  - Returned by a server answering a SUMMON message to
    ///    indicate that it is summoning that user.
    ///
    RPL_SUMMONING(&'a [u8]),

    /// 346    RPL_INVITELIST
    ///       "<channel> <invitemask>"
    RPL_INVITELIST(&'a [u8]),

    /// 347    RPL_ENDOFINVITELIST
    ///       "<channel> :End of channel invite list"
    ///
    ///  - When listing the 'invitations masks' for a given channel,
    ///    a server is required to send the list back using the
    ///    RPL_INVITELIST and RPL_ENDOFINVITELIST messages.  A
    ///    separate RPL_INVITELIST is sent for each active mask.
    ///    After the masks have been listed (or if none present) a
    ///    RPL_ENDOFINVITELIST MUST be sent.
    ///
    RPL_ENDOFINVITELIST(&'a [u8]),

    /// 348    RPL_EXCEPTLIST
    ///       "<channel> <exceptionmask>"
    RPL_EXCEPTLIST(&'a [u8]),

    /// 349    RPL_ENDOFEXCEPTLIST
    ///       "<channel> :End of channel exception list"
    ///
    ///  - When listing the 'exception masks' for a given channel,
    ///    a server is required to send the list back using the
    ///    RPL_EXCEPTLIST and RPL_ENDOFEXCEPTLIST messages.  A
    ///    separate RPL_EXCEPTLIST is sent for each active mask.
    ///    After the masks have been listed (or if none present)
    ///    a RPL_ENDOFEXCEPTLIST MUST be sent.
    ///
    RPL_ENDOFEXCEPTLIST(&'a [u8]),

    /// 351    RPL_VERSION
    ///       "<version>.<debuglevel> <server> :<comments>"
    ///
    ///  - Reply by the server showing its version details.
    ///    The <version> is the version of the software being
    ///    used (including any patchlevel revisions) and the
    ///    <debuglevel> is used to indicate if the server is
    ///    running in "debug mode".
    ///
    ///    The "comments" field may contain any comments about
    ///    the version or further version details.
    ///
    RPL_VERSION(&'a [u8]),

    /// 352    RPL_WHOREPLY
    ///       "<channel> <user> <host> <server> <nick>
    ///       ( "H" / "G" > ["*"] [ ( "@" / "+" ) ]
    ///       :<hopcount> <real name>"
    ///
    RPL_WHOREPLY(&'a [u8]),

    /// 315    RPL_ENDOFWHO
    ///       "<name> :End of WHO list"
    ///
    ///  - The RPL_WHOREPLY and RPL_ENDOFWHO pair are used
    ///    to answer a WHO message.  The RPL_WHOREPLY is only
    ///    sent if there is an appropriate match to the WHO
    ///    query.  If there is a list of parameters supplied
    ///    with a WHO message, a RPL_ENDOFWHO MUST be sent
    ///    after processing each list item with <name> being
    ///    the item.
    ///
    RPL_ENDOFWHO(&'a [u8]),

    /// 353    RPL_NAMREPLY
    ///       "( "=" / "*" / "@" ) <channel>
    ///        :[ "@" / "+" ] <nick> *( " " [ "@" / "+" ] <nick> )
    ///  - "@" is used for secret channels, "*" for private
    ///    channels, and "=" for others (public channels).
    ///
    RPL_NAMREPLY(&'a [u8]),

    /// 366    RPL_ENDOFNAMES
    ///       "<channel> :End of NAMES list"
    ///
    ///  - To reply to a NAMES message, a reply pair consisting
    ///    of RPL_NAMREPLY and RPL_ENDOFNAMES is sent by the
    ///    server back to the client.  If there is no channel
    ///    found as in the query, then only RPL_ENDOFNAMES is
    ///    returned.  The exception to this is when a NAMES
    ///    message is sent with no parameters and all visible
    ///    channels and contents are sent back in a series of
    ///    RPL_NAMEREPLY messages with a RPL_ENDOFNAMES to mark
    ///    the end.
    ///
    RPL_ENDOFNAMES(&'a [u8]),

    /// 364    RPL_LINKS
    ///       "<mask> <server> :<hopcount> <server info>"
    RPL_LINKS(&'a [u8]),

    /// 365    RPL_ENDOFLINKS
    ///       "<mask> :End of LINKS list"
    ///
    ///  - In replying to the LINKS message, a server MUST send
    ///    replies back using the RPL_LINKS numeric and mark the
    ///    end of the list using an RPL_ENDOFLINKS reply.
    ///
    RPL_ENDOFLINKS(&'a [u8]),

    /// 367    RPL_BANLIST
    ///       "<channel> <banmask>"
    RPL_BANLIST(&'a [u8]),

    /// 368    RPL_ENDOFBANLIST
    ///       "<channel> :End of channel ban list"
    ///
    ///  - When listing the active 'bans' for a given channel,
    ///    a server is required to send the list back using the
    ///    RPL_BANLIST and RPL_ENDOFBANLIST messages.  A separate
    ///    RPL_BANLIST is sent for each active banmask.  After the
    ///    banmasks have been listed (or if none present) a
    ///    RPL_ENDOFBANLIST MUST be sent.
    ///
    RPL_ENDOFBANLIST(&'a [u8]),

    /// 371    RPL_INFO
    ///       ":<string>"
    RPL_INFO(&'a [u8]),

    /// 374    RPL_ENDOFINFO
    ///       ":End of INFO list"
    ///
    ///  - A server responding to an INFO message is required to
    ///    send all its 'info' in a series of RPL_INFO messages
    ///    with a RPL_ENDOFINFO reply to indicate the end of the
    ///    replies.
    ///
    RPL_ENDOFINFO(&'a [u8]),

    /// 375    RPL_MOTDSTART
    ///       ":- <server> Message of the day - "
    RPL_MOTDSTART(&'a [u8]),

    /// 372    RPL_MOTD
    ///       ":- <text>"
    RPL_MOTD(&'a [u8]),

    /// 376    RPL_ENDOFMOTD
    ///       ":End of MOTD command"
    ///
    ///  - When responding to the MOTD message and the MOTD file
    ///    is found, the file is displayed line by line, with
    ///    each line no longer than 80 characters, using
    ///    RPL_MOTD format replies.  These MUST be surrounded
    ///    by a RPL_MOTDSTART (before the RPL_MOTDs) and an
    ///    RPL_ENDOFMOTD (after).
    ///
    RPL_ENDOFMOTD(&'a [u8]),

    /// 381    RPL_YOUREOPER
    ///       ":You are now an IRC operator"
    ///
    ///  - RPL_YOUREOPER is sent back to a client which has
    ///    just successfully issued an OPER message and gained
    ///    operator status.
    ///
    RPL_YOUREOPER(&'a [u8]),

    /// 382    RPL_REHASHING
    ///       "<config file> :Rehashing"
    ///
    ///  - If the REHASH option is used and an operator sends
    ///    a REHASH message, an RPL_REHASHING is sent back to
    ///    the operator.
    ///
    RPL_REHASHING(&'a [u8]),

    /// 383    RPL_YOURESERVICE
    ///       "You are service <servicename>"
    ///
    ///  - Sent by the server to a service upon successful
    ///    registration.
    ///
    RPL_YOURESERVICE(&'a [u8]),

    /// 391    RPL_TIME
    ///       "<server> :<string showing server's local time>"
    ///
    ///  - When replying to the TIME message, a server MUST send
    ///    the reply using the RPL_TIME format above.  The string
    ///    showing the time need only contain the correct day and
    ///    time there.  There is no further requirement for the
    ///    time string.
    ///
    RPL_TIME(&'a [u8]),

    /// 392    RPL_USERSSTART
    ///       ":UserID   Terminal  Host"
    RPL_USERSSTART(&'a [u8]),

    /// 393    RPL_USERS
    ///       ":<username> <ttyline> <hostname>"
    RPL_USERS(&'a [u8]),

    /// 394    RPL_ENDOFUSERS
    ///       ":End of users"
    RPL_ENDOFUSERS(&'a [u8]),

    /// 395    RPL_NOUSERS
    ///       ":Nobody logged in"
    ///
    ///  - If the USERS message is handled by a server, the
    ///    replies RPL_USERSTART, RPL_USERS, RPL_ENDOFUSERS and
    ///    RPL_NOUSERS are used.  RPL_USERSSTART MUST be sent
    ///    first, following by either a sequence of RPL_USERS
    ///    or a single RPL_NOUSER.  Following this is
    ///    RPL_ENDOFUSERS.
    ///
    RPL_NOUSERS(&'a [u8]),

    /// 200    RPL_TRACELINK
    ///       "Link <version & debug level> <destination>
    ///        <next server> V<protocol version>
    ///        <link uptime in seconds> <backstream sendq>
    ///        <upstream sendq>"
    RPL_TRACELINK(&'a [u8]),

    /// 201    RPL_TRACECONNECTING
    ///       "Try. <class> <server>"
    RPL_TRACECONNECTING(&'a [u8]),

    /// 202    RPL_TRACEHANDSHAKE
    ///       "H.S. <class> <server>"
    RPL_TRACEHANDSHAKE(&'a [u8]),

    /// 203    RPL_TRACEUNKNOWN
    ///       "???? <class> [<client IP address in dot form>]"
    RPL_TRACEUNKNOWN(&'a [u8]),

    /// 204    RPL_TRACEOPERATOR
    ///       "Oper <class> <nick>"
    RPL_TRACEOPERATOR(&'a [u8]),

    /// 205    RPL_TRACEUSER
    ///       "User <class> <nick>"
    RPL_TRACEUSER(&'a [u8]),

    /// 206    RPL_TRACESERVER
    ///       "Serv <class> <int>S <int>C <server>
    ///        <nick!user|*!*>@<host|server> V<protocol version>"
    RPL_TRACESERVER(&'a [u8]),

    /// 207    RPL_TRACESERVICE
    ///       "Service <class> <name> <type> <active type>"
    RPL_TRACESERVICE(&'a [u8]),

    /// 208    RPL_TRACENEWTYPE
    ///       "<newtype> 0 <client name>"
    RPL_TRACENEWTYPE(&'a [u8]),

    /// 209    RPL_TRACECLASS
    ///       "Class <class> <count>"
    RPL_TRACECLASS(&'a [u8]),

    /// 210    RPL_TRACERECONNECT
    ///       Unused.
    RPL_TRACERECONNECT(&'a [u8]),

    /// 261    RPL_TRACELOG
    ///       "File <logfile> <debug level>"
    RPL_TRACELOG(&'a [u8]),

    /// 262    RPL_TRACEEND
    ///       "<server name> <version & debug level> :End of TRACE"
    ///
    ///  - The RPL_TRACE* are all returned by the server in
    ///    response to the TRACE message.  How many are
    ///    returned is dependent on the TRACE message and
    ///    whether it was sent by an operator or not.  There
    ///    is no predefined order for which occurs first.
    ///    Replies RPL_TRACEUNKNOWN, RPL_TRACECONNECTING and
    ///    RPL_TRACEHANDSHAKE are all used for connections
    ///    which have not been fully established and are either
    ///    unknown, still attempting to connect or in the
    ///    process of completing the 'server handshake'.
    ///    RPL_TRACELINK is sent by any server which handles
    ///    a TRACE message and has to pass it on to another
    ///    server.  The list of RPL_TRACELINKs sent in
    ///    response to a TRACE command traversing the IRC
    ///    network should reflect the actual connectivity of
    ///    the servers themselves along that path.
    ///
    ///    RPL_TRACENEWTYPE is to be used for any connection
    ///    which does not fit in the other categories but is
    ///    being displayed anyway.
    ///    RPL_TRACEEND is sent to indicate the end of the list.
    ///
    RPL_TRACEEND(&'a [u8]),

    /// 211    RPL_STATSLINKINFO
    ///       "<linkname> <sendq> <sent messages>
    ///        <sent Kbytes> <received messages>
    ///        <received Kbytes> <time open>"
    ///
    ///  - reports statistics on a connection.  <linkname>
    ///    identifies the particular connection, <sendq> is
    ///    the amount of data that is queued and waiting to be
    ///    sent <sent messages> the number of messages sent,
    ///    and <sent Kbytes> the amount of data sent, in
    ///    Kbytes. <received messages> and <received Kbytes>
    ///    are the equivalent of <sent messages> and <sent
    ///    Kbytes> for received data, respectively.  <time
    ///    open> indicates how long ago the connection was
    ///    opened, in seconds.
    ///
    RPL_STATSLINKINFO(&'a [u8]),

    /// 212    RPL_STATSCOMMANDS
    ///       "<command> <count> <byte count> <remote count>"
    ///
    ///  - reports statistics on commands usage.
    ///
    RPL_STATSCOMMANDS(&'a [u8]),

    /// 219    RPL_ENDOFSTATS
    ///       "<stats letter> :End of STATS report"
    ///
    RPL_ENDOFSTATS(&'a [u8]),

    /// 242    RPL_STATSUPTIME
    ///       ":Server Up %d days %d:%02d:%02d"
    ///
    ///  - reports the server uptime.
    ///
    RPL_STATSUPTIME(&'a [u8]),

    /// 243    RPL_STATSOLINE
    ///       "O <hostmask> * <name>"
    ///
    ///  - reports the allowed hosts from where user may become IRC
    ///    operators.
    ///
    RPL_STATSOLINE(&'a [u8]),

    /// 221    RPL_UMODEIS
    ///       "<user mode string>"
    ///
    ///  - To answer a query about a client's own mode,
    ///    RPL_UMODEIS is sent back.
    ///
    RPL_UMODEIS(&'a [u8]),

    /// 234    RPL_SERVLIST
    ///       "<name> <server> <mask> <type> <hopcount> <info>"
    ///
    RPL_SERVLIST(&'a [u8]),

    /// 235    RPL_SERVLISTEND
    ///       "<mask> <type> :End of service listing"
    ///
    ///  - When listing services in reply to a SERVLIST message,
    ///    a server is required to send the list back using the
    ///    RPL_SERVLIST and RPL_SERVLISTEND messages.  A separate
    ///    RPL_SERVLIST is sent for each service.  After the
    ///    services have been listed (or if none present) a
    ///    RPL_SERVLISTEND MUST be sent.
    ///
    RPL_SERVLISTEND(&'a [u8]),

    /// 251    RPL_LUSERCLIENT
    ///       ":There are <integer> users and <integer>
    ///        services on <integer> servers"
    RPL_LUSERCLIENT(&'a [u8]),

    /// 252    RPL_LUSEROP
    ///       "<integer> :operator(s) online"
    RPL_LUSEROP(&'a [u8]),

    /// 253    RPL_LUSERUNKNOWN
    ///       "<integer> :unknown connection(s)"
    RPL_LUSERUNKNOWN(&'a [u8]),

    /// 254    RPL_LUSERCHANNELS
    ///       "<integer> :channels formed"
    RPL_LUSERCHANNELS(&'a [u8]),

    /// 255    RPL_LUSERME
    ///       ":I have <integer> clients and <integer>
    ///         servers"
    ///
    ///  - In processing an LUSERS message, the server
    ///    sends a set of replies from RPL_LUSERCLIENT,
    ///    RPL_LUSEROP, RPL_USERUNKNOWN,
    ///    RPL_LUSERCHANNELS and RPL_LUSERME.  When
    ///    replying, a server MUST send back
    ///    RPL_LUSERCLIENT and RPL_LUSERME.  The other
    ///    replies are only sent back if a non-zero count
    ///    is found for them.
    ///
    RPL_LUSERME(&'a [u8]),

    /// 256    RPL_ADMINME
    ///       "<server> :Administrative info"
    RPL_ADMINME(&'a [u8]),

    /// 257    RPL_ADMINLOC1
    ///       ":<admin info>"
    RPL_ADMINLOC1(&'a [u8]),

    /// 258    RPL_ADMINLOC2
    ///       ":<admin info>"
    RPL_ADMINLOC2(&'a [u8]),

    /// 259    RPL_ADMINEMAIL
    ///       ":<admin info>"
    ///
    ///  - When replying to an ADMIN message, a server
    ///    is expected to use replies RPL_ADMINME
    ///    through to RPL_ADMINEMAIL and provide a text
    ///    message with each.  For RPL_ADMINLOC1 a
    ///    description of what city, state and country
    ///    the server is in is expected, followed by
    ///    details of the institution (RPL_ADMINLOC2)
    ///
    ///    and finally the administrative contact for the
    ///    server (an email address here is REQUIRED)
    ///    in RPL_ADMINEMAIL.
    ///
    RPL_ADMINEMAIL(&'a [u8]),

    /// 263    RPL_TRYAGAIN
    ///       "<command> :Please wait a while and try again."
    ///
    ///  - When a server drops a command without processing it,
    ///    it MUST use the reply RPL_TRYAGAIN to inform the
    ///    originating client.
    ///
    RPL_TRYAGAIN(&'a [u8]),

    /// 401    ERR_NOSUCHNICK
    ///       "<nickname> :No such nick/channel"
    ///
    ///   - Used to indicate the nickname parameter supplied to a
    ///     command is currently unused.
    ///
    ERR_NOSUCHNICK(&'a [u8]),

    /// 402    ERR_NOSUCHSERVER
    ///       "<server name> :No such server"
    ///
    ///  - Used to indicate the server name given currently
    ///    does not exist.
    ///
    ERR_NOSUCHSERVER(&'a [u8]),

    /// 403    ERR_NOSUCHCHANNEL
    ///       "<channel name> :No such channel"
    ///
    ///  - Used to indicate the given channel name is invalid.
    ///
    ERR_NOSUCHCHANNEL(&'a [u8]),

    /// 404    ERR_CANNOTSENDTOCHAN
    ///       "<channel name> :Cannot send to channel"
    ///
    ///  - Sent to a user who is either (a) not on a channel
    ///    which is mode +n or (b) not a chanop (or mode +v) on
    ///    a channel which has mode +m set or where the user is
    ///    banned and is trying to send a PRIVMSG message to
    ///    that channel.
    ///
    ERR_CANNOTSENDTOCHAN(&'a [u8]),

    /// 405    ERR_TOOMANYCHANNELS
    ///       "<channel name> :You have joined too many channels"
    ///
    ///  - Sent to a user when they have joined the maximum
    ///    number of allowed channels and they try to join
    ///    another channel.
    ///
    ERR_TOOMANYCHANNELS(&'a [u8]),

    /// 406    ERR_WASNOSUCHNICK
    ///       "<nickname> :There was no such nickname"
    ///
    ///  - Returned by WHOWAS to indicate there is no history
    ///    information for that nickname.
    ///
    ERR_WASNOSUCHNICK(&'a [u8]),

    /// 407    ERR_TOOMANYTARGETS
    ///       "<target> :<error code> recipients. <abort message>"
    ///
    ///  - Returned to a client which is attempting to send a
    ///    PRIVMSG/NOTICE using the user@host destination format
    ///    and for a user@host which has several occurrences.
    ///
    ///  - Returned to a client which trying to send a
    ///    PRIVMSG/NOTICE to too many recipients.
    ///
    ///  - Returned to a client which is attempting to JOIN a safe
    ///    channel using the shortname when there are more than one
    ///    such channel.
    ///
    ERR_TOOMANYTARGETS(&'a [u8]),

    /// 408    ERR_NOSUCHSERVICE
    ///       "<service name> :No such service"
    ///
    ///  - Returned to a client which is attempting to send a SQUERY
    ///    to a service which does not exist.
    ///
    ERR_NOSUCHSERVICE(&'a [u8]),

    /// 409    ERR_NOORIGIN
    ///       ":No origin specified"
    ///
    ///  - PING or PONG message missing the originator parameter.
    ///
    ERR_NOORIGIN(&'a [u8]),

    /// 411    ERR_NORECIPIENT
    ///       ":No recipient given (<command>)"
    ERR_NORECIPIENT(&'a [u8]),

    /// 412    ERR_NOTEXTTOSEND
    ///       ":No text to send"
    ERR_NOTEXTTOSEND(&'a [u8]),

    /// 413    ERR_NOTOPLEVEL
    ///       "<mask> :No toplevel domain specified"
    ERR_NOTOPLEVEL(&'a [u8]),

    /// 414    ERR_WILDTOPLEVEL
    ///       "<mask> :Wildcard in toplevel domain"
    ERR_WILDTOPLEVEL(&'a [u8]),

    /// 415    ERR_BADMASK
    ///       "<mask> :Bad Server/host mask"
    ///
    ///  - 412 - 415 are returned by PRIVMSG to indicate that
    ///    the message wasn't delivered for some reason.
    ///    ERR_NOTOPLEVEL and ERR_WILDTOPLEVEL are errors that
    ///    are returned when an invalid use of
    ///    "PRIVMSG $<server>" or "PRIVMSG #<host>" is attempted.
    ///
    ERR_BADMASK(&'a [u8]),

    /// 421    ERR_UNKNOWNCOMMAND
    ///       "<command> :Unknown command"
    ///
    ///  - Returned to a registered client to indicate that the
    ///    command sent is unknown by the server.
    ///
    ERR_UNKNOWNCOMMAND(&'a [u8]),

    /// 422    ERR_NOMOTD
    ///       ":MOTD File is missing"
    ///
    ///  - Server's MOTD file could not be opened by the server.
    ///
    ERR_NOMOTD(&'a [u8]),

    /// 423    ERR_NOADMININFO
    ///       "<server> :No administrative info available"
    ///
    ///  - Returned by a server in response to an ADMIN message
    ///    when there is an error in finding the appropriate
    ///    information.
    ///
    ERR_NOADMININFO(&'a [u8]),

    /// 424    ERR_FILEERROR
    ///       ":File error doing <file op> on <file>"
    ///
    ///  - Generic error message used to report a failed file
    ///    operation during the processing of a message.
    ///
    ERR_FILEERROR(&'a [u8]),

    /// 431    ERR_NONICKNAMEGIVEN
    ///       ":No nickname given"
    ///
    ///  - Returned when a nickname parameter expected for a
    ///    command and isn't found.
    ///
    ERR_NONICKNAMEGIVEN(&'a [u8]),

    /// 432    ERR_ERRONEUSNICKNAME
    ///       "<nick> :Erroneous nickname"
    ///
    ///  - Returned after receiving a NICK message which contains
    ///    characters which do not fall in the defined set.  See
    ///    section 2.3.1 for details on valid nicknames.
    ///
    ERR_ERRONEUSNICKNAME(&'a [u8]),

    /// 433    ERR_NICKNAMEINUSE
    ///       "<nick> :Nickname is already in use"
    ///
    ///  - Returned when a NICK message is processed that results
    ///    in an attempt to change to a currently existing
    ///    nickname.
    ///
    ERR_NICKNAMEINUSE(&'a [u8]),

    /// 436    ERR_NICKCOLLISION
    ///       "<nick> :Nickname collision KILL from <user>@<host>"
    ///
    ///  - Returned by a server to a client when it detects a
    ///    nickname collision (registered of a NICK that
    ///    already exists by another server).
    ///
    ERR_NICKCOLLISION(&'a [u8]),

    /// 437    ERR_UNAVAILRESOURCE
    ///       "<nick/channel> :Nick/channel is temporarily unavailable"
    ///
    ///  - Returned by a server to a user trying to join a channel
    ///    currently blocked by the channel delay mechanism.
    ///
    ///  - Returned by a server to a user trying to change nickname
    ///    when the desired nickname is blocked by the nick delay
    ///    mechanism.
    ///
    ERR_UNAVAILRESOURCE(&'a [u8]),

    /// 441    ERR_USERNOTINCHANNEL
    ///       "<nick> <channel> :They aren't on that channel"
    ///
    ///  - Returned by the server to indicate that the target
    ///    user of the command is not on the given channel.
    ///
    ERR_USERNOTINCHANNEL(&'a [u8]),

    /// 442    ERR_NOTONCHANNEL
    ///       "<channel> :You're not on that channel"
    ///
    ///  - Returned by the server whenever a client tries to
    ///    perform a channel affecting command for which the
    ///    client isn't a member.
    ///
    ERR_NOTONCHANNEL(&'a [u8]),

    /// 443    ERR_USERONCHANNEL
    ///       "<user> <channel> :is already on channel"
    ///
    ///  - Returned when a client tries to invite a user to a
    ///    channel they are already on.
    ///
    ERR_USERONCHANNEL(&'a [u8]),

    /// 444    ERR_NOLOGIN
    ///       "<user> :User not logged in"
    ///
    ///  - Returned by the summon after a SUMMON command for a
    ///    user was unable to be performed since they were not
    ///    logged in.
    ///
    ERR_NOLOGIN(&'a [u8]),

    /// 445    ERR_SUMMONDISABLED
    ///       ":SUMMON has been disabled"
    ///
    ///  - Returned as a response to the SUMMON command.  MUST be
    ///    returned by any server which doesn't implement it.
    ///
    ERR_SUMMONDISABLED(&'a [u8]),

    /// 446    ERR_USERSDISABLED
    ///       ":USERS has been disabled"
    ///
    ///  - Returned as a response to the USERS command.  MUST be
    ///    returned by any server which does not implement it.
    ///
    ERR_USERSDISABLED(&'a [u8]),

    /// 451    ERR_NOTREGISTERED
    ///       ":You have not registered"
    ///
    ///  - Returned by the server to indicate that the client
    ///    MUST be registered before the server will allow it
    ///    to be parsed in detail.
    ///
    ERR_NOTREGISTERED(&'a [u8]),

    /// 461    ERR_NEEDMOREPARAMS
    ///       "<command> :Not enough parameters"
    ///
    ///  - Returned by the server by numerous commands to
    ///    indicate to the client that it didn't supply enough
    ///    parameters.
    ///
    ERR_NEEDMOREPARAMS(&'a [u8]),

    /// 462    ERR_ALREADYREGISTRED
    ///       ":Unauthorized command (already registered)"
    ///
    ///  - Returned by the server to any link which tries to
    ///    change part of the registered details (such as
    ///    password or user details from second USER message).
    ///
    ERR_ALREADYREGISTRED(&'a [u8]),

    /// 463    ERR_NOPERMFORHOST
    ///       ":Your host isn't among the privileged"
    ///
    ///  - Returned to a client which attempts to register with
    ///    a server which does not been setup to allow
    ///    connections from the host the attempted connection
    ///    is tried.
    ///
    ERR_NOPERMFORHOST(&'a [u8]),

    /// 464    ERR_PASSWDMISMATCH
    ///       ":Password incorrect"
    ///
    ///  - Returned to indicate a failed attempt at registering
    ///    a connection for which a password was required and
    ///    was either not given or incorrect.
    ///
    ERR_PASSWDMISMATCH(&'a [u8]),

    /// 465    ERR_YOUREBANNEDCREEP
    ///       ":You are banned from this server"
    ///
    ///  - Returned after an attempt to connect and register
    ///    yourself with a server which has been setup to
    ///    explicitly deny connections to you.
    ///
    ERR_YOUREBANNEDCREEP(&'a [u8]),

    /// 466    ERR_YOUWILLBEBANNED
    ///
    ///  - Sent by a server to a user to inform that access to the
    ///    server will soon be denied.
    ///
    ERR_YOUWILLBEBANNED(&'a [u8]),

    /// 467    ERR_KEYSET
    ///       "<channel> :Channel key already set"
    ERR_KEYSET(&'a [u8]),

    /// 471    ERR_CHANNELISFULL
    ///       "<channel> :Cannot join channel (+l)"
    ERR_CHANNELISFULL(&'a [u8]),

    /// 472    ERR_UNKNOWNMODE
    ///       "<char> :is unknown mode char to me for <channel>"
    ERR_UNKNOWNMODE(&'a [u8]),

    /// 473    ERR_INVITEONLYCHAN
    ///       "<channel> :Cannot join channel (+i)"
    ERR_INVITEONLYCHAN(&'a [u8]),

    /// 474    ERR_BANNEDFROMCHAN
    ///       "<channel> :Cannot join channel (+b)"
    ERR_BANNEDFROMCHAN(&'a [u8]),

    /// 475    ERR_BADCHANNELKEY
    ///       "<channel> :Cannot join channel (+k)"
    ERR_BADCHANNELKEY(&'a [u8]),

    /// 476    ERR_BADCHANMASK
    ///       "<channel> :Bad Channel Mask"
    ERR_BADCHANMASK(&'a [u8]),

    /// 477    ERR_NOCHANMODES
    ///       "<channel> :Channel doesn't support modes"
    ERR_NOCHANMODES(&'a [u8]),

    /// 478    ERR_BANLISTFULL
    ///       "<channel> <char> :Channel list is full"
    ///
    ERR_BANLISTFULL(&'a [u8]),

    /// 481    ERR_NOPRIVILEGES
    ///       ":Permission Denied- You're not an IRC operator"
    ///
    ///  - Any command requiring operator privileges to operate
    ///    MUST return this error to indicate the attempt was
    ///    unsuccessful.
    ///
    ERR_NOPRIVILEGES(&'a [u8]),

    /// 482    ERR_CHANOPRIVSNEEDED
    ///       "<channel> :You're not channel operator"
    ///
    ///  - Any command requiring 'chanop' privileges (such as
    ///    MODE messages) MUST return this error if the client
    ///    making the attempt is not a chanop on the specified
    ///    channel.
    ///
    ERR_CHANOPRIVSNEEDED(&'a [u8]),

    /// 483    ERR_CANTKILLSERVER
    ///       ":You can't kill a server!"
    ///
    ///  - Any attempts to use the KILL command on a server
    ///    are to be refused and this error returned directly
    ///    to the client.
    ///
    ERR_CANTKILLSERVER(&'a [u8]),

    /// 484    ERR_RESTRICTED
    ///       ":Your connection is restricted!"
    ///
    ///  - Sent by the server to a user upon connection to indicate
    ///    the restricted nature of the connection (user mode "+r").
    ///
    ERR_RESTRICTED(&'a [u8]),

    /// 485    ERR_UNIQOPPRIVSNEEDED
    ///       ":You're not the original channel operator"
    ///
    ///  - Any MODE requiring "channel creator" privileges MUST
    ///    return this error if the client making the attempt is not
    ///    a chanop on the specified channel.
    ///
    ERR_UNIQOPPRIVSNEEDED(&'a [u8]),

    /// 491    ERR_NOOPERHOST
    ///       ":No O-lines for your host"
    ///
    ///  - If a client sends an OPER message and the server has
    ///    not been configured to allow connections from the
    ///    client's host as an operator, this error MUST be
    ///    returned.
    ///
    ERR_NOOPERHOST(&'a [u8]),

    /// 501    ERR_UMODEUNKNOWNFLAG
    ///       ":Unknown MODE flag"
    ///
    ///  - Returned by the server to indicate that a MODE
    ///    message was sent with a nickname parameter and that
    ///    the a mode flag sent was not recognized.
    ///
    ERR_UMODEUNKNOWNFLAG(&'a [u8]),

    /// 502    ERR_USERSDONTMATCH
    ///       ":Cannot change mode for other users"
    ///
    ///  - Error sent to any user trying to view or change the
    ///    user mode for a user other than themselves.
    ///
    ERR_USERSDONTMATCH(&'a [u8]),

}

impl<'a> Reply<'a> {
    pub fn from_message(msg: &'a Message) -> Option<Reply<'a>> {
        use self::Reply::*;
        match msg.command() {
            "001" => msg.last().map(|&e| RPL_WELCOME(Borrowed(e))),
            "002" => msg.last().map(|&e| RPL_YOURHOST(Borrowed(e))),
            "003" => msg.last().map(|&e| RPL_CREATED(Borrowed(e))),
            "004" => msg.last().map(|&e| RPL_MYINFO(Borrowed(e))),
            "005" => msg.last().map(|&e| RPL_BOUNCE(Borrowed(e))),
            "302" => msg.last().map(|&e| RPL_USERHOST(Borrowed(e))),
            "303" => msg.elements().last().map(|&e| RPL_ISON(Borrowed(e))),
            "301" => msg.elements().last().map(|&e| RPL_AWAY(Borrowed(e))),
            "305" => msg.elements().last().map(|&e| RPL_UNAWAY(Borrowed(e))),
            "306" => msg.elements().last().map(|&e| RPL_NOWAWAY(Borrowed(e))),
            "311" => msg.elements().last().map(|&e| RPL_WHOISUSER(Borrowed(e))),
            "312" => msg.elements().last().map(|&e| RPL_WHOISSERVER(Borrowed(e))),
            "313" => msg.elements().last().map(|&e| RPL_WHOISOPERATOR(Borrowed(e))),
            "317" => msg.elements().last().map(|&e| RPL_WHOISIDLE(Borrowed(e))),
            "318" => msg.elements().last().map(|&e| RPL_ENDOFWHOIS(Borrowed(e))),
            "319" => msg.elements().last().map(|&e| RPL_WHOISCHANNELS(Borrowed(e))),
            "314" => msg.elements().last().map(|&e| RPL_WHOWASUSER(Borrowed(e))),
            "369" => msg.elements().last().map(|&e| RPL_ENDOFWHOWAS(Borrowed(e))),
            "321" => Some(RPL_LISTSTART),
            "322" => msg.elements().last().map(|&e| RPL_LIST(Borrowed(e))),
            "323" => msg.elements().last().map(|&e| RPL_LISTEND(Borrowed(e))),
            "325" => msg.elements().last().map(|&e| RPL_UNIQOPIS(Borrowed(e))),
            "324" => msg.elements().last().map(|&e| RPL_CHANNELMODEIS(Borrowed(e))),
            "331" => msg.elements().last().map(|&e| RPL_NOTOPIC(Borrowed(e))),
            "332" => msg.elements().last().map(|&e| RPL_TOPIC(Borrowed(e))),
            "341" => msg.elements().last().map(|&e| RPL_INVITING(Borrowed(e))),
            "342" => msg.elements().last().map(|&e| RPL_SUMMONING(Borrowed(e))),
            "346" => msg.elements().last().map(|&e| RPL_INVITELIST(Borrowed(e))),
            "347" => msg.elements().last().map(|&e| RPL_ENDOFINVITELIST(Borrowed(e))),
            "348" => msg.elements().last().map(|&e| RPL_EXCEPTLIST(Borrowed(e))),
            "349" => msg.elements().last().map(|&e| RPL_ENDOFEXCEPTLIST(Borrowed(e))),
            "351" => msg.elements().last().map(|&e| RPL_VERSION(Borrowed(e))),
            "352" => msg.elements().last().map(|&e| RPL_WHOREPLY(Borrowed(e))),
            "315" => msg.elements().last().map(|&e| RPL_ENDOFWHO(Borrowed(e))),
            "353" => msg.elements().last().map(|&e| RPL_NAMREPLY(Borrowed(e))),
            "366" => msg.elements().last().map(|&e| RPL_ENDOFNAMES(Borrowed(e))),
            "364" => msg.elements().last().map(|&e| RPL_LINKS(Borrowed(e))),
            "365" => msg.elements().last().map(|&e| RPL_ENDOFLINKS(Borrowed(e))),
            "367" => msg.elements().last().map(|&e| RPL_BANLIST(Borrowed(e))),
            "368" => msg.elements().last().map(|&e| RPL_ENDOFBANLIST(Borrowed(e))),
            "371" => msg.elements().last().map(|&e| RPL_INFO(Borrowed(e))),
            "374" => msg.elements().last().map(|&e| RPL_ENDOFINFO(Borrowed(e))),
            "375" => msg.elements().last().map(|&e| RPL_MOTDSTART(Borrowed(e))),
            "372" => msg.elements().last().map(|&e| RPL_MOTD(Borrowed(e))),
            "376" => msg.elements().last().map(|&e| RPL_ENDOFMOTD(Borrowed(e))),
            "381" => msg.elements().last().map(|&e| RPL_YOUREOPER(Borrowed(e))),
            "382" => msg.elements().last().map(|&e| RPL_REHASHING(Borrowed(e))),
            "383" => msg.elements().last().map(|&e| RPL_YOURESERVICE(Borrowed(e))),
            "391" => msg.elements().last().map(|&e| RPL_TIME(Borrowed(e))),
            "392" => msg.elements().last().map(|&e| RPL_USERSSTART(Borrowed(e))),
            "393" => msg.elements().last().map(|&e| RPL_USERS(Borrowed(e))),
            "394" => msg.elements().last().map(|&e| RPL_ENDOFUSERS(Borrowed(e))),
            "395" => msg.elements().last().map(|&e| RPL_NOUSERS(Borrowed(e))),
            "200" => msg.elements().last().map(|&e| RPL_TRACELINK(Borrowed(e))),
            "201" => msg.elements().last().map(|&e| RPL_TRACECONNECTING(Borrowed(e))),
            "202" => msg.elements().last().map(|&e| RPL_TRACEHANDSHAKE(Borrowed(e))),
            "203" => msg.elements().last().map(|&e| RPL_TRACEUNKNOWN(Borrowed(e))),
            "204" => msg.elements().last().map(|&e| RPL_TRACEOPERATOR(Borrowed(e))),
            "205" => msg.elements().last().map(|&e| RPL_TRACEUSER(Borrowed(e))),
            "206" => msg.elements().last().map(|&e| RPL_TRACESERVER(Borrowed(e))),
            "207" => msg.elements().last().map(|&e| RPL_TRACESERVICE(Borrowed(e))),
            "208" => msg.elements().last().map(|&e| RPL_TRACENEWTYPE(Borrowed(e))),
            "209" => msg.elements().last().map(|&e| RPL_TRACECLASS(Borrowed(e))),
            "210" => msg.elements().last().map(|&e| RPL_TRACERECONNECT(Borrowed(e))),
            "261" => msg.elements().last().map(|&e| RPL_TRACELOG(Borrowed(e))),
            "262" => msg.elements().last().map(|&e| RPL_TRACEEND(Borrowed(e))),
            "211" => msg.elements().last().map(|&e| RPL_STATSLINKINFO(Borrowed(e))),
            "212" => msg.elements().last().map(|&e| RPL_STATSCOMMANDS(Borrowed(e))),
            "219" => msg.elements().last().map(|&e| RPL_ENDOFSTATS(Borrowed(e))),
            "242" => msg.elements().last().map(|&e| RPL_STATSUPTIME(Borrowed(e))),
            "243" => msg.elements().last().map(|&e| RPL_STATSOLINE(Borrowed(e))),
            "221" => msg.elements().last().map(|&e| RPL_UMODEIS(Borrowed(e))),
            "234" => msg.elements().last().map(|&e| RPL_SERVLIST(Borrowed(e))),
            "235" => msg.elements().last().map(|&e| RPL_SERVLISTEND(Borrowed(e))),
            "251" => msg.elements().last().map(|&e| RPL_LUSERCLIENT(Borrowed(e))),
            "252" => msg.elements().last().map(|&e| RPL_LUSEROP(Borrowed(e))),
            "253" => msg.elements().last().map(|&e| RPL_LUSERUNKNOWN(Borrowed(e))),
            "254" => msg.elements().last().map(|&e| RPL_LUSERCHANNELS(Borrowed(e))),
            "255" => msg.elements().last().map(|&e| RPL_LUSERME(Borrowed(e))),
            "256" => msg.elements().last().map(|&e| RPL_ADMINME(Borrowed(e))),
            "257" => msg.elements().last().map(|&e| RPL_ADMINLOC1(Borrowed(e))),
            "258" => msg.elements().last().map(|&e| RPL_ADMINLOC2(Borrowed(e))),
            "259" => msg.elements().last().map(|&e| RPL_ADMINEMAIL(Borrowed(e))),
            "263" => msg.elements().last().map(|&e| RPL_TRYAGAIN(Borrowed(e))),
            "401" => msg.elements().last().map(|&e| ERR_NOSUCHNICK(Borrowed(e))),
            "402" => msg.elements().last().map(|&e| ERR_NOSUCHSERVER(Borrowed(e))),
            "403" => msg.elements().last().map(|&e| ERR_NOSUCHCHANNEL(Borrowed(e))),
            "404" => msg.elements().last().map(|&e| ERR_CANNOTSENDTOCHAN(Borrowed(e))),
            "405" => msg.elements().last().map(|&e| ERR_TOOMANYCHANNELS(Borrowed(e))),
            "406" => msg.elements().last().map(|&e| ERR_WASNOSUCHNICK(Borrowed(e))),
            "407" => msg.elements().last().map(|&e| ERR_TOOMANYTARGETS(Borrowed(e))),
            "408" => msg.elements().last().map(|&e| ERR_NOSUCHSERVICE(Borrowed(e))),
            "409" => msg.elements().last().map(|&e| ERR_NOORIGIN(Borrowed(e))),
            "411" => msg.elements().last().map(|&e| ERR_NORECIPIENT(Borrowed(e))),
            "412" => msg.elements().last().map(|&e| ERR_NOTEXTTOSEND(Borrowed(e))),
            "413" => msg.elements().last().map(|&e| ERR_NOTOPLEVEL(Borrowed(e))),
            "414" => msg.elements().last().map(|&e| ERR_WILDTOPLEVEL(Borrowed(e))),
            "415" => msg.elements().last().map(|&e| ERR_BADMASK(Borrowed(e))),
            "421" => msg.elements().last().map(|&e| ERR_UNKNOWNCOMMAND(Borrowed(e))),
            "422" => msg.elements().last().map(|&e| ERR_NOMOTD(Borrowed(e))),
            "423" => msg.elements().last().map(|&e| ERR_NOADMININFO(Borrowed(e))),
            "424" => msg.elements().last().map(|&e| ERR_FILEERROR(Borrowed(e))),
            "431" => msg.elements().last().map(|&e| ERR_NONICKNAMEGIVEN(Borrowed(e))),
            "432" => msg.elements().last().map(|&e| ERR_ERRONEUSNICKNAME(Borrowed(e))),
            "433" => msg.elements().last().map(|&e| ERR_NICKNAMEINUSE(Borrowed(e))),
            "436" => msg.elements().last().map(|&e| ERR_NICKCOLLISION(Borrowed(e))),
            "437" => msg.elements().last().map(|&e| ERR_UNAVAILRESOURCE(Borrowed(e))),
            "441" => msg.elements().last().map(|&e| ERR_USERNOTINCHANNEL(Borrowed(e))),
            "442" => msg.elements().last().map(|&e| ERR_NOTONCHANNEL(Borrowed(e))),
            "443" => msg.elements().last().map(|&e| ERR_USERONCHANNEL(Borrowed(e))),
            "444" => msg.elements().last().map(|&e| ERR_NOLOGIN(Borrowed(e))),
            "445" => msg.elements().last().map(|&e| ERR_SUMMONDISABLED(Borrowed(e))),
            "446" => msg.elements().last().map(|&e| ERR_USERSDISABLED(Borrowed(e))),
            "451" => msg.elements().last().map(|&e| ERR_NOTREGISTERED(Borrowed(e))),
            "461" => msg.elements().last().map(|&e| ERR_NEEDMOREPARAMS(Borrowed(e))),
            "462" => msg.elements().last().map(|&e| ERR_ALREADYREGISTRED(Borrowed(e))),
            "463" => msg.elements().last().map(|&e| ERR_NOPERMFORHOST(Borrowed(e))),
            "464" => msg.elements().last().map(|&e| ERR_PASSWDMISMATCH(Borrowed(e))),
            "465" => msg.elements().last().map(|&e| ERR_YOUREBANNEDCREEP(Borrowed(e))),
            "466" => msg.elements().last().map(|&e| ERR_YOUWILLBEBANNED(Borrowed(e))),
            "467" => msg.elements().last().map(|&e| ERR_KEYSET(Borrowed(e))),
            "471" => msg.elements().last().map(|&e| ERR_CHANNELISFULL(Borrowed(e))),
            "472" => msg.elements().last().map(|&e| ERR_UNKNOWNMODE(Borrowed(e))),
            "473" => msg.elements().last().map(|&e| ERR_INVITEONLYCHAN(Borrowed(e))),
            "474" => msg.elements().last().map(|&e| ERR_BANNEDFROMCHAN(Borrowed(e))),
            "475" => msg.elements().last().map(|&e| ERR_BADCHANNELKEY(Borrowed(e))),
            "476" => msg.elements().last().map(|&e| ERR_BADCHANMASK(Borrowed(e))),
            "477" => msg.elements().last().map(|&e| ERR_NOCHANMODES(Borrowed(e))),
            "478" => msg.elements().last().map(|&e| ERR_BANLISTFULL(Borrowed(e))),
            "481" => msg.elements().last().map(|&e| ERR_NOPRIVILEGES(Borrowed(e))),
            "482" => msg.elements().last().map(|&e| ERR_CHANOPRIVSNEEDED(Borrowed(e))),
            "483" => msg.elements().last().map(|&e| ERR_CANTKILLSERVER(Borrowed(e))),
            "484" => msg.elements().last().map(|&e| ERR_RESTRICTED(Borrowed(e))),
            "485" => msg.elements().last().map(|&e| ERR_UNIQOPPRIVSNEEDED(Borrowed(e))),
            "491" => msg.elements().last().map(|&e| ERR_NOOPERHOST(Borrowed(e))),
            "501" => msg.elements().last().map(|&e| ERR_UMODEUNKNOWNFLAG(Borrowed(e))),
            "502" => msg.elements().last().map(|&e| ERR_USERSDONTMATCH(Borrowed(e))),
            _ => None
        }
     }

    pub fn to_message(&'a self) -> Message {
        use self::Reply::*;
        match self {
            &RPL_WELCOME(ref s) => Message::format(None, Borrowed("001"), vec![], s),
            &RPL_YOURHOST(ref s) => Message::format(None, Borrowed("002"), vec![], s),
            &RPL_CREATED(ref s) => Message::format(None, Borrowed("003"), vec![], s),
            &RPL_MYINFO(ref s) => Message::format(None, Borrowed("004"), vec![], s),
            &RPL_BOUNCE(ref s) => Message::format(None, Borrowed("005"), vec![], s),
            &RPL_USERHOST(ref s) => Message::format(None, Borrowed("302"), vec![], s),
            &RPL_ISON(ref s) => Message::format(None, Borrowed("303"), vec![], s),
            &RPL_AWAY(ref s) => Message::format(None, Borrowed("301"), vec![], s),
            &RPL_UNAWAY(ref s) => Message::format(None, Borrowed("305"), vec![], s),
            &RPL_NOWAWAY(ref s) => Message::format(None, Borrowed("306"), vec![], s),
            &RPL_WHOISUSER(ref s) => Message::format(None, Borrowed("311"), vec![], s),
            &RPL_WHOISSERVER(ref s) => Message::format(None, Borrowed("312"), vec![], s),
            &RPL_WHOISOPERATOR(ref s) => Message::format(None, Borrowed("313"), vec![], s),
            &RPL_WHOISIDLE(ref s) => Message::format(None, Borrowed("317"), vec![], s),
            &RPL_ENDOFWHOIS(ref s) => Message::format(None, Borrowed("318"), vec![], s),
            &RPL_WHOISCHANNELS(ref s) => Message::format(None, Borrowed("319"), vec![], s),
            &RPL_WHOWASUSER(ref s) => Message::format(None, Borrowed("314"), vec![], s),
            &RPL_ENDOFWHOWAS(ref s) => Message::format(None, Borrowed("369"), vec![], s),
            &RPL_LISTSTART => Message::format(None, Borrowed("321"), vec![], None),
            &RPL_LIST(ref s) => Message::format(None, Borrowed("322"), vec![], s),
            &RPL_LISTEND(ref s) => Message::format(None, Borrowed("323"), vec![], s),
            &RPL_UNIQOPIS(ref s) => Message::format(None, Borrowed("325"), vec![], s),
            &RPL_CHANNELMODEIS(ref s) => Message::format(None, Borrowed("324"), vec![], s),
            &RPL_NOTOPIC(ref s) => Message::format(None, Borrowed("331"), vec![], s),
            &RPL_TOPIC(ref s) => Message::format(None, Borrowed("332"), vec![], s),
            &RPL_INVITING(ref s) => Message::format(None, Borrowed("341"), vec![], s),
            &RPL_SUMMONING(ref s) => Message::format(None, Borrowed("342"), vec![], s),
            &RPL_INVITELIST(ref s) => Message::format(None, Borrowed("346"), vec![], s),
            &RPL_ENDOFINVITELIST(ref s) => Message::format(None, Borrowed("347"), vec![], s),
            &RPL_EXCEPTLIST(ref s) => Message::format(None, Borrowed("348"), vec![], s),
            &RPL_ENDOFEXCEPTLIST(ref s) => Message::format(None, Borrowed("349"), vec![], s),
            &RPL_VERSION(ref s) => Message::format(None, Borrowed("351"), vec![], s),
            &RPL_WHOREPLY(ref s) => Message::format(None, Borrowed("352"), vec![], s),
            &RPL_ENDOFWHO(ref s) => Message::format(None, Borrowed("315"), vec![], s),
            &RPL_NAMREPLY(ref s) => Message::format(None, Borrowed("353"), vec![], s),
            &RPL_ENDOFNAMES(ref s) => Message::format(None, Borrowed("366"), vec![], s),
            &RPL_LINKS(ref s) => Message::format(None, Borrowed("364"), vec![], s),
            &RPL_ENDOFLINKS(ref s) => Message::format(None, Borrowed("365"), vec![], s),
            &RPL_BANLIST(ref s) => Message::format(None, Borrowed("367"), vec![], s),
            &RPL_ENDOFBANLIST(ref s) => Message::format(None, Borrowed("368"), vec![], s),
            &RPL_INFO(ref s) => Message::format(None, Borrowed("371"), vec![], s),
            &RPL_ENDOFINFO(ref s) => Message::format(None, Borrowed("374"), vec![], s),
            &RPL_MOTDSTART(ref s) => Message::format(None, Borrowed("375"), vec![], s),
            &RPL_MOTD(ref s) => Message::format(None, Borrowed("372"), vec![], s),
            &RPL_ENDOFMOTD(ref s) => Message::format(None, Borrowed("376"), vec![], s),
            &RPL_YOUREOPER(ref s) => Message::format(None, Borrowed("381"), vec![], s),
            &RPL_REHASHING(ref s) => Message::format(None, Borrowed("382"), vec![], s),
            &RPL_YOURESERVICE(ref s) => Message::format(None, Borrowed("383"), vec![], s),
            &RPL_TIME(ref s) => Message::format(None, Borrowed("391"), vec![], s),
            &RPL_USERSSTART(ref s) => Message::format(None, Borrowed("392"), vec![], s),
            &RPL_USERS(ref s) => Message::format(None, Borrowed("393"), vec![], s),
            &RPL_ENDOFUSERS(ref s) => Message::format(None, Borrowed("394"), vec![], s),
            &RPL_NOUSERS(ref s) => Message::format(None, Borrowed("395"), vec![], s),
            &RPL_TRACELINK(ref s) => Message::format(None, Borrowed("200"), vec![], s),
            &RPL_TRACECONNECTING(ref s) => Message::format(None, Borrowed("201"), vec![], s),
            &RPL_TRACEHANDSHAKE(ref s) => Message::format(None, Borrowed("202"), vec![], s),
            &RPL_TRACEUNKNOWN(ref s) => Message::format(None, Borrowed("203"), vec![], s),
            &RPL_TRACEOPERATOR(ref s) => Message::format(None, Borrowed("204"), vec![], s),
            &RPL_TRACEUSER(ref s) => Message::format(None, Borrowed("205"), vec![], s),
            &RPL_TRACESERVER(ref s) => Message::format(None, Borrowed("206"), vec![], s),
            &RPL_TRACESERVICE(ref s) => Message::format(None, Borrowed("207"), vec![], s),
            &RPL_TRACENEWTYPE(ref s) => Message::format(None, Borrowed("208"), vec![], s),
            &RPL_TRACECLASS(ref s) => Message::format(None, Borrowed("209"), vec![], s),
            &RPL_TRACERECONNECT(ref s) => Message::format(None, Borrowed("210"), vec![], s),
            &RPL_TRACELOG(ref s) => Message::format(None, Borrowed("261"), vec![], s),
            &RPL_TRACEEND(ref s) => Message::format(None, Borrowed("262"), vec![], s),
            &RPL_STATSLINKINFO(ref s) => Message::format(None, Borrowed("211"), vec![], s),
            &RPL_STATSCOMMANDS(ref s) => Message::format(None, Borrowed("212"), vec![], s),
            &RPL_ENDOFSTATS(ref s) => Message::format(None, Borrowed("219"), vec![], s),
            &RPL_STATSUPTIME(ref s) => Message::format(None, Borrowed("242"), vec![], s),
            &RPL_STATSOLINE(ref s) => Message::format(None, Borrowed("243"), vec![], s),
            &RPL_UMODEIS(ref s) => Message::format(None, Borrowed("221"), vec![], s),
            &RPL_SERVLIST(ref s) => Message::format(None, Borrowed("234"), vec![], s),
            &RPL_SERVLISTEND(ref s) => Message::format(None, Borrowed("235"), vec![], s),
            &RPL_LUSERCLIENT(ref s) => Message::format(None, Borrowed("251"), vec![], s),
            &RPL_LUSEROP(ref s) => Message::format(None, Borrowed("252"), vec![], s),
            &RPL_LUSERUNKNOWN(ref s) => Message::format(None, Borrowed("253"), vec![], s),
            &RPL_LUSERCHANNELS(ref s) => Message::format(None, Borrowed("254"), vec![], s),
            &RPL_LUSERME(ref s) => Message::format(None, Borrowed("255"), vec![], s),
            &RPL_ADMINME(ref s) => Message::format(None, Borrowed("256"), vec![], s),
            &RPL_ADMINLOC1(ref s) => Message::format(None, Borrowed("257"), vec![], s),
            &RPL_ADMINLOC2(ref s) => Message::format(None, Borrowed("258"), vec![], s),
            &RPL_ADMINEMAIL(ref s) => Message::format(None, Borrowed("259"), vec![], s),
            &RPL_TRYAGAIN(ref s) => Message::format(None, Borrowed("263"), vec![], s),
            &ERR_NOSUCHNICK(ref s) => Message::format(None, Borrowed("401"), vec![], s),
            &ERR_NOSUCHSERVER(ref s) => Message::format(None, Borrowed("402"), vec![], s),
            &ERR_NOSUCHCHANNEL(ref s) => Message::format(None, Borrowed("403"), vec![], s),
            &ERR_CANNOTSENDTOCHAN(ref s) => Message::format(None, Borrowed("404"), vec![], s),
            &ERR_TOOMANYCHANNELS(ref s) => Message::format(None, Borrowed("405"), vec![], s),
            &ERR_WASNOSUCHNICK(ref s) => Message::format(None, Borrowed("406"), vec![], s),
            &ERR_TOOMANYTARGETS(ref s) => Message::format(None, Borrowed("407"), vec![], s),
            &ERR_NOSUCHSERVICE(ref s) => Message::format(None, Borrowed("408"), vec![], s),
            &ERR_NOORIGIN(ref s) => Message::format(None, Borrowed("409"), vec![], s),
            &ERR_NORECIPIENT(ref s) => Message::format(None, Borrowed("411"), vec![], s),
            &ERR_NOTEXTTOSEND(ref s) => Message::format(None, Borrowed("412"), vec![], s),
            &ERR_NOTOPLEVEL(ref s) => Message::format(None, Borrowed("413"), vec![], s),
            &ERR_WILDTOPLEVEL(ref s) => Message::format(None, Borrowed("414"), vec![], s),
            &ERR_BADMASK(ref s) => Message::format(None, Borrowed("415"), vec![], s),
            &ERR_UNKNOWNCOMMAND(ref s) => Message::format(None, Borrowed("421"), vec![], s),
            &ERR_NOMOTD(ref s) => Message::format(None, Borrowed("422"), vec![], s),
            &ERR_NOADMININFO(ref s) => Message::format(None, Borrowed("423"), vec![], s),
            &ERR_FILEERROR(ref s) => Message::format(None, Borrowed("424"), vec![], s),
            &ERR_NONICKNAMEGIVEN(ref s) => Message::format(None, Borrowed("431"), vec![], s),
            &ERR_ERRONEUSNICKNAME(ref s) => Message::format(None, Borrowed("432"), vec![], s),
            &ERR_NICKNAMEINUSE(ref s) => Message::format(None, Borrowed("433"), vec![], s),
            &ERR_NICKCOLLISION(ref s) => Message::format(None, Borrowed("436"), vec![], s),
            &ERR_UNAVAILRESOURCE(ref s) => Message::format(None, Borrowed("437"), vec![], s),
            &ERR_USERNOTINCHANNEL(ref s) => Message::format(None, Borrowed("441"), vec![], s),
            &ERR_NOTONCHANNEL(ref s) => Message::format(None, Borrowed("442"), vec![], s),
            &ERR_USERONCHANNEL(ref s) => Message::format(None, Borrowed("443"), vec![], s),
            &ERR_NOLOGIN(ref s) => Message::format(None, Borrowed("444"), vec![], s),
            &ERR_SUMMONDISABLED(ref s) => Message::format(None, Borrowed("445"), vec![], s),
            &ERR_USERSDISABLED(ref s) => Message::format(None, Borrowed("446"), vec![], s),
            &ERR_NOTREGISTERED(ref s) => Message::format(None, Borrowed("451"), vec![], s),
            &ERR_NEEDMOREPARAMS(ref s) => Message::format(None, Borrowed("461"), vec![], s),
            &ERR_ALREADYREGISTRED(ref s) => Message::format(None, Borrowed("462"), vec![], s),
            &ERR_NOPERMFORHOST(ref s) => Message::format(None, Borrowed("463"), vec![], s),
            &ERR_PASSWDMISMATCH(ref s) => Message::format(None, Borrowed("464"), vec![], s),
            &ERR_YOUREBANNEDCREEP(ref s) => Message::format(None, Borrowed("465"), vec![], s),
            &ERR_YOUWILLBEBANNED(ref s) => Message::format(None, Borrowed("466"), vec![], s),
            &ERR_KEYSET(ref s) => Message::format(None, Borrowed("467"), vec![], s),
            &ERR_CHANNELISFULL(ref s) => Message::format(None, Borrowed("471"), vec![], s),
            &ERR_UNKNOWNMODE(ref s) => Message::format(None, Borrowed("472"), vec![], s),
            &ERR_INVITEONLYCHAN(ref s) => Message::format(None, Borrowed("473"), vec![], s),
            &ERR_BANNEDFROMCHAN(ref s) => Message::format(None, Borrowed("474"), vec![], s),
            &ERR_BADCHANNELKEY(ref s) => Message::format(None, Borrowed("475"), vec![], s),
            &ERR_BADCHANMASK(ref s) => Message::format(None, Borrowed("476"), vec![], s),
            &ERR_NOCHANMODES(ref s) => Message::format(None, Borrowed("477"), vec![], s),
            &ERR_BANLISTFULL(ref s) => Message::format(None, Borrowed("478"), vec![], s),
            &ERR_NOPRIVILEGES(ref s) => Message::format(None, Borrowed("481"), vec![], s),
            &ERR_CHANOPRIVSNEEDED(ref s) => Message::format(None, Borrowed("482"), vec![], s),
            &ERR_CANTKILLSERVER(ref s) => Message::format(None, Borrowed("483"), vec![], s),
            &ERR_RESTRICTED(ref s) => Message::format(None, Borrowed("484"), vec![], s),
            &ERR_UNIQOPPRIVSNEEDED(ref s) => Message::format(None, Borrowed("485"), vec![], s),
            &ERR_NOOPERHOST(ref s) => Message::format(None, Borrowed("491"), vec![], s),
            &ERR_UMODEUNKNOWNFLAG(ref s) => Message::format(None, Borrowed("501"), vec![], s),
            &ERR_USERSDONTMATCH(ref s) => Message::format(None, Borrowed("502"), vec![], s),
        }
     }
}*/
