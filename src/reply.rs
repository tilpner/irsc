use std::str::FromStr;
use std::borrow::{ Cow, ToOwned };
use std::borrow::Cow::*;

use ::{ Result, IrscError };
use ::message::{ MsgType, Message };

pub type CS<'a> = Cow<'a, str>;

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Reply<'a> {
    /// 001    RPL_WELCOME
    ///       "Welcome to the Internet Relay Network
    ///        <nick>!<user>@<host>"
    RPL_WELCOME(CS<'a>),

    /// 002    RPL_YOURHOST
    ///       "Your host is <servername>, running version <ver>"
    RPL_YOURHOST(CS<'a>),

    /// 003    RPL_CREATED
    ///       "This server was created <date>"
    RPL_CREATED(CS<'a>),

    /// 004    RPL_MYINFO
    ///       "<servername> <version> <available user modes>
    ///        <available channel modes>"
    ///
    ///  - The server sends Replies 001 to 004 to a user upon
    ///    successful registration.
    ///
    RPL_MYINFO(CS<'a>),

    /// 005    RPL_BOUNCE
    ///       "Try server <server name>, port <port number>"
    ///
    ///  - Sent by the server to a user to suggest an alternative
    ///    server.  This is often used when the connection is
    ///    refused because the server is already full.
    ///
    RPL_BOUNCE(CS<'a>),

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
    RPL_USERHOST(CS<'a>),

    /// 303    RPL_ISON
    ///       ":*1<nick> *( " " <nick> )"
    ///
    ///  - Reply format used by ISON to list replies to the
    ///    query list.
    ///
    RPL_ISON(CS<'a>),

    /// 301    RPL_AWAY
    ///       "<nick> :<away message>"
    RPL_AWAY(CS<'a>),

    /// 305    RPL_UNAWAY
    ///       ":You are no longer marked as being away"
    RPL_UNAWAY(CS<'a>),

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
    RPL_NOWAWAY(CS<'a>),

    /// 311    RPL_WHOISUSER
    ///       "<nick> <user> <host> * :<real name>"
    RPL_WHOISUSER(CS<'a>),

    /// 312    RPL_WHOISSERVER
    ///       "<nick> <server> :<server info>"
    RPL_WHOISSERVER(CS<'a>),

    /// 313    RPL_WHOISOPERATOR
    ///       "<nick> :is an IRC operator"
    RPL_WHOISOPERATOR(CS<'a>),

    /// 317    RPL_WHOISIDLE
    ///       "<nick> <integer> :seconds idle"
    RPL_WHOISIDLE(CS<'a>),

    /// 318    RPL_ENDOFWHOIS
    ///       "<nick> :End of WHOIS list"
    RPL_ENDOFWHOIS(CS<'a>),

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
    RPL_WHOISCHANNELS(CS<'a>),

    /// 314    RPL_WHOWASUSER
    ///       "<nick> <user> <host> * :<real name>"
    RPL_WHOWASUSER(CS<'a>),

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
    RPL_ENDOFWHOWAS(CS<'a>),

    /// 321    RPL_LISTSTART
    ///       Obsolete. Not used.
    ///
    RPL_LISTSTART,

    /// 322    RPL_LIST
    ///       "<channel> <# visible> :<topic>"
    RPL_LIST(CS<'a>),

    /// 323    RPL_LISTEND
    ///       ":End of LIST"
    ///
    ///  - Replies RPL_LIST, RPL_LISTEND mark the actual replies
    ///    with data and end of the server's response to a LIST
    ///    command.  If there are no channels available to return,
    ///    only the end reply MUST be sent.
    ///
    RPL_LISTEND(CS<'a>),

    /// 325    RPL_UNIQOPIS
    ///       "<channel> <nickname>"
    ///
    RPL_UNIQOPIS(CS<'a>),

    /// 324    RPL_CHANNELMODEIS
    ///       "<channel> <mode> <mode params>"
    ///
    RPL_CHANNELMODEIS(CS<'a>),

    /// 331    RPL_NOTOPIC
    ///       "<channel> :No topic is set"
    RPL_NOTOPIC(CS<'a>),

    /// 332    RPL_TOPIC
    ///       "<channel> :<topic>"
    ///
    ///  - When sending a TOPIC message to determine the
    ///    channel topic, one of two replies is sent.  If
    ///    the topic is set, RPL_TOPIC is sent back else
    ///    RPL_NOTOPIC.
    ///
    RPL_TOPIC(CS<'a>),

    /// 341    RPL_INVITING
    ///       "<channel> <nick>"
    ///
    ///  - Returned by the server to indicate that the
    ///    attempted INVITE message was successful and is
    ///    being passed onto the end client.
    ///
    RPL_INVITING(CS<'a>),

    /// 342    RPL_SUMMONING
    ///       "<user> :Summoning user to IRC"
    ///
    ///  - Returned by a server answering a SUMMON message to
    ///    indicate that it is summoning that user.
    ///
    RPL_SUMMONING(CS<'a>),

    /// 346    RPL_INVITELIST
    ///       "<channel> <invitemask>"
    RPL_INVITELIST(CS<'a>),

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
    RPL_ENDOFINVITELIST(CS<'a>),

    /// 348    RPL_EXCEPTLIST
    ///       "<channel> <exceptionmask>"
    RPL_EXCEPTLIST(CS<'a>),

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
    RPL_ENDOFEXCEPTLIST(CS<'a>),

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
    RPL_VERSION(CS<'a>),

    /// 352    RPL_WHOREPLY
    ///       "<channel> <user> <host> <server> <nick>
    ///       ( "H" / "G" > ["*"] [ ( "@" / "+" ) ]
    ///       :<hopcount> <real name>"
    ///
    RPL_WHOREPLY(CS<'a>),

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
    RPL_ENDOFWHO(CS<'a>),

    /// 353    RPL_NAMREPLY
    ///       "( "=" / "*" / "@" ) <channel>
    ///        :[ "@" / "+" ] <nick> *( " " [ "@" / "+" ] <nick> )
    ///  - "@" is used for secret channels, "*" for private
    ///    channels, and "=" for others (public channels).
    ///
    RPL_NAMREPLY(CS<'a>),

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
    RPL_ENDOFNAMES(CS<'a>),

    /// 364    RPL_LINKS
    ///       "<mask> <server> :<hopcount> <server info>"
    RPL_LINKS(CS<'a>),

    /// 365    RPL_ENDOFLINKS
    ///       "<mask> :End of LINKS list"
    ///
    ///  - In replying to the LINKS message, a server MUST send
    ///    replies back using the RPL_LINKS numeric and mark the
    ///    end of the list using an RPL_ENDOFLINKS reply.
    ///
    RPL_ENDOFLINKS(CS<'a>),

    /// 367    RPL_BANLIST
    ///       "<channel> <banmask>"
    RPL_BANLIST(CS<'a>),

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
    RPL_ENDOFBANLIST(CS<'a>),

    /// 371    RPL_INFO
    ///       ":<string>"
    RPL_INFO(CS<'a>),

    /// 374    RPL_ENDOFINFO
    ///       ":End of INFO list"
    ///
    ///  - A server responding to an INFO message is required to
    ///    send all its 'info' in a series of RPL_INFO messages
    ///    with a RPL_ENDOFINFO reply to indicate the end of the
    ///    replies.
    ///
    RPL_ENDOFINFO(CS<'a>),

    /// 375    RPL_MOTDSTART
    ///       ":- <server> Message of the day - "
    RPL_MOTDSTART(CS<'a>),

    /// 372    RPL_MOTD
    ///       ":- <text>"
    RPL_MOTD(CS<'a>),

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
    RPL_ENDOFMOTD(CS<'a>),

    /// 381    RPL_YOUREOPER
    ///       ":You are now an IRC operator"
    ///
    ///  - RPL_YOUREOPER is sent back to a client which has
    ///    just successfully issued an OPER message and gained
    ///    operator status.
    ///
    RPL_YOUREOPER(CS<'a>),

    /// 382    RPL_REHASHING
    ///       "<config file> :Rehashing"
    ///
    ///  - If the REHASH option is used and an operator sends
    ///    a REHASH message, an RPL_REHASHING is sent back to
    ///    the operator.
    ///
    RPL_REHASHING(CS<'a>),

    /// 383    RPL_YOURESERVICE
    ///       "You are service <servicename>"
    ///
    ///  - Sent by the server to a service upon successful
    ///    registration.
    ///
    RPL_YOURESERVICE(CS<'a>),

    /// 391    RPL_TIME
    ///       "<server> :<string showing server's local time>"
    ///
    ///  - When replying to the TIME message, a server MUST send
    ///    the reply using the RPL_TIME format above.  The string
    ///    showing the time need only contain the correct day and
    ///    time there.  There is no further requirement for the
    ///    time string.
    ///
    RPL_TIME(CS<'a>),

    /// 392    RPL_USERSSTART
    ///       ":UserID   Terminal  Host"
    RPL_USERSSTART(CS<'a>),

    /// 393    RPL_USERS
    ///       ":<username> <ttyline> <hostname>"
    RPL_USERS(CS<'a>),

    /// 394    RPL_ENDOFUSERS
    ///       ":End of users"
    RPL_ENDOFUSERS(CS<'a>),

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
    RPL_NOUSERS(CS<'a>),

    /// 200    RPL_TRACELINK
    ///       "Link <version & debug level> <destination>
    ///        <next server> V<protocol version>
    ///        <link uptime in seconds> <backstream sendq>
    ///        <upstream sendq>"
    RPL_TRACELINK(CS<'a>),

    /// 201    RPL_TRACECONNECTING
    ///       "Try. <class> <server>"
    RPL_TRACECONNECTING(CS<'a>),

    /// 202    RPL_TRACEHANDSHAKE
    ///       "H.S. <class> <server>"
    RPL_TRACEHANDSHAKE(CS<'a>),

    /// 203    RPL_TRACEUNKNOWN
    ///       "???? <class> [<client IP address in dot form>]"
    RPL_TRACEUNKNOWN(CS<'a>),

    /// 204    RPL_TRACEOPERATOR
    ///       "Oper <class> <nick>"
    RPL_TRACEOPERATOR(CS<'a>),

    /// 205    RPL_TRACEUSER
    ///       "User <class> <nick>"
    RPL_TRACEUSER(CS<'a>),

    /// 206    RPL_TRACESERVER
    ///       "Serv <class> <int>S <int>C <server>
    ///        <nick!user|*!*>@<host|server> V<protocol version>"
    RPL_TRACESERVER(CS<'a>),

    /// 207    RPL_TRACESERVICE
    ///       "Service <class> <name> <type> <active type>"
    RPL_TRACESERVICE(CS<'a>),

    /// 208    RPL_TRACENEWTYPE
    ///       "<newtype> 0 <client name>"
    RPL_TRACENEWTYPE(CS<'a>),

    /// 209    RPL_TRACECLASS
    ///       "Class <class> <count>"
    RPL_TRACECLASS(CS<'a>),

    /// 210    RPL_TRACERECONNECT
    ///       Unused.
    RPL_TRACERECONNECT(CS<'a>),

    /// 261    RPL_TRACELOG
    ///       "File <logfile> <debug level>"
    RPL_TRACELOG(CS<'a>),

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
    RPL_TRACEEND(CS<'a>),

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
    RPL_STATSLINKINFO(CS<'a>),

    /// 212    RPL_STATSCOMMANDS
    ///       "<command> <count> <byte count> <remote count>"
    ///
    ///  - reports statistics on commands usage.
    ///
    RPL_STATSCOMMANDS(CS<'a>),

    /// 219    RPL_ENDOFSTATS
    ///       "<stats letter> :End of STATS report"
    ///
    RPL_ENDOFSTATS(CS<'a>),

    /// 242    RPL_STATSUPTIME
    ///       ":Server Up %d days %d:%02d:%02d"
    ///
    ///  - reports the server uptime.
    ///
    RPL_STATSUPTIME(CS<'a>),

    /// 243    RPL_STATSOLINE
    ///       "O <hostmask> * <name>"
    ///
    ///  - reports the allowed hosts from where user may become IRC
    ///    operators.
    ///
    RPL_STATSOLINE(CS<'a>),

    /// 221    RPL_UMODEIS
    ///       "<user mode string>"
    ///
    ///  - To answer a query about a client's own mode,
    ///    RPL_UMODEIS is sent back.
    ///
    RPL_UMODEIS(CS<'a>),

    /// 234    RPL_SERVLIST
    ///       "<name> <server> <mask> <type> <hopcount> <info>"
    ///
    RPL_SERVLIST(CS<'a>),

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
    RPL_SERVLISTEND(CS<'a>),

    /// 251    RPL_LUSERCLIENT
    ///       ":There are <integer> users and <integer>
    ///        services on <integer> servers"
    RPL_LUSERCLIENT(CS<'a>),

    /// 252    RPL_LUSEROP
    ///       "<integer> :operator(s) online"
    RPL_LUSEROP(CS<'a>),

    /// 253    RPL_LUSERUNKNOWN
    ///       "<integer> :unknown connection(s)"
    RPL_LUSERUNKNOWN(CS<'a>),

    /// 254    RPL_LUSERCHANNELS
    ///       "<integer> :channels formed"
    RPL_LUSERCHANNELS(CS<'a>),

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
    RPL_LUSERME(CS<'a>),

    /// 256    RPL_ADMINME
    ///       "<server> :Administrative info"
    RPL_ADMINME(CS<'a>),

    /// 257    RPL_ADMINLOC1
    ///       ":<admin info>"
    RPL_ADMINLOC1(CS<'a>),

    /// 258    RPL_ADMINLOC2
    ///       ":<admin info>"
    RPL_ADMINLOC2(CS<'a>),

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
    RPL_ADMINEMAIL(CS<'a>),

    /// 263    RPL_TRYAGAIN
    ///       "<command> :Please wait a while and try again."
    ///
    ///  - When a server drops a command without processing it,
    ///    it MUST use the reply RPL_TRYAGAIN to inform the
    ///    originating client.
    ///
    RPL_TRYAGAIN(CS<'a>),

    /// 401    ERR_NOSUCHNICK
    ///       "<nickname> :No such nick/channel"
    ///
    ///   - Used to indicate the nickname parameter supplied to a
    ///     command is currently unused.
    ///
    ERR_NOSUCHNICK(CS<'a>),

    /// 402    ERR_NOSUCHSERVER
    ///       "<server name> :No such server"
    ///
    ///  - Used to indicate the server name given currently
    ///    does not exist.
    ///
    ERR_NOSUCHSERVER(CS<'a>),

    /// 403    ERR_NOSUCHCHANNEL
    ///       "<channel name> :No such channel"
    ///
    ///  - Used to indicate the given channel name is invalid.
    ///
    ERR_NOSUCHCHANNEL(CS<'a>),

    /// 404    ERR_CANNOTSENDTOCHAN
    ///       "<channel name> :Cannot send to channel"
    ///
    ///  - Sent to a user who is either (a) not on a channel
    ///    which is mode +n or (b) not a chanop (or mode +v) on
    ///    a channel which has mode +m set or where the user is
    ///    banned and is trying to send a PRIVMSG message to
    ///    that channel.
    ///
    ERR_CANNOTSENDTOCHAN(CS<'a>),

    /// 405    ERR_TOOMANYCHANNELS
    ///       "<channel name> :You have joined too many channels"
    ///
    ///  - Sent to a user when they have joined the maximum
    ///    number of allowed channels and they try to join
    ///    another channel.
    ///
    ERR_TOOMANYCHANNELS(CS<'a>),

    /// 406    ERR_WASNOSUCHNICK
    ///       "<nickname> :There was no such nickname"
    ///
    ///  - Returned by WHOWAS to indicate there is no history
    ///    information for that nickname.
    ///
    ERR_WASNOSUCHNICK(CS<'a>),

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
    ERR_TOOMANYTARGETS(CS<'a>),

    /// 408    ERR_NOSUCHSERVICE
    ///       "<service name> :No such service"
    ///
    ///  - Returned to a client which is attempting to send a SQUERY
    ///    to a service which does not exist.
    ///
    ERR_NOSUCHSERVICE(CS<'a>),

    /// 409    ERR_NOORIGIN
    ///       ":No origin specified"
    ///
    ///  - PING or PONG message missing the originator parameter.
    ///
    ERR_NOORIGIN(CS<'a>),

    /// 411    ERR_NORECIPIENT
    ///       ":No recipient given (<command>)"
    ERR_NORECIPIENT(CS<'a>),

    /// 412    ERR_NOTEXTTOSEND
    ///       ":No text to send"
    ERR_NOTEXTTOSEND(CS<'a>),

    /// 413    ERR_NOTOPLEVEL
    ///       "<mask> :No toplevel domain specified"
    ERR_NOTOPLEVEL(CS<'a>),

    /// 414    ERR_WILDTOPLEVEL
    ///       "<mask> :Wildcard in toplevel domain"
    ERR_WILDTOPLEVEL(CS<'a>),

    /// 415    ERR_BADMASK
    ///       "<mask> :Bad Server/host mask"
    ///
    ///  - 412 - 415 are returned by PRIVMSG to indicate that
    ///    the message wasn't delivered for some reason.
    ///    ERR_NOTOPLEVEL and ERR_WILDTOPLEVEL are errors that
    ///    are returned when an invalid use of
    ///    "PRIVMSG $<server>" or "PRIVMSG #<host>" is attempted.
    ///
    ERR_BADMASK(CS<'a>),

    /// 421    ERR_UNKNOWNCOMMAND
    ///       "<command> :Unknown command"
    ///
    ///  - Returned to a registered client to indicate that the
    ///    command sent is unknown by the server.
    ///
    ERR_UNKNOWNCOMMAND(CS<'a>),

    /// 422    ERR_NOMOTD
    ///       ":MOTD File is missing"
    ///
    ///  - Server's MOTD file could not be opened by the server.
    ///
    ERR_NOMOTD(CS<'a>),

    /// 423    ERR_NOADMININFO
    ///       "<server> :No administrative info available"
    ///
    ///  - Returned by a server in response to an ADMIN message
    ///    when there is an error in finding the appropriate
    ///    information.
    ///
    ERR_NOADMININFO(CS<'a>),

    /// 424    ERR_FILEERROR
    ///       ":File error doing <file op> on <file>"
    ///
    ///  - Generic error message used to report a failed file
    ///    operation during the processing of a message.
    ///
    ERR_FILEERROR(CS<'a>),

    /// 431    ERR_NONICKNAMEGIVEN
    ///       ":No nickname given"
    ///
    ///  - Returned when a nickname parameter expected for a
    ///    command and isn't found.
    ///
    ERR_NONICKNAMEGIVEN(CS<'a>),

    /// 432    ERR_ERRONEUSNICKNAME
    ///       "<nick> :Erroneous nickname"
    ///
    ///  - Returned after receiving a NICK message which contains
    ///    characters which do not fall in the defined set.  See
    ///    section 2.3.1 for details on valid nicknames.
    ///
    ERR_ERRONEUSNICKNAME(CS<'a>),

    /// 433    ERR_NICKNAMEINUSE
    ///       "<nick> :Nickname is already in use"
    ///
    ///  - Returned when a NICK message is processed that results
    ///    in an attempt to change to a currently existing
    ///    nickname.
    ///
    ERR_NICKNAMEINUSE(CS<'a>),

    /// 436    ERR_NICKCOLLISION
    ///       "<nick> :Nickname collision KILL from <user>@<host>"
    ///
    ///  - Returned by a server to a client when it detects a
    ///    nickname collision (registered of a NICK that
    ///    already exists by another server).
    ///
    ERR_NICKCOLLISION(CS<'a>),

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
    ERR_UNAVAILRESOURCE(CS<'a>),

    /// 441    ERR_USERNOTINCHANNEL
    ///       "<nick> <channel> :They aren't on that channel"
    ///
    ///  - Returned by the server to indicate that the target
    ///    user of the command is not on the given channel.
    ///
    ERR_USERNOTINCHANNEL(CS<'a>),

    /// 442    ERR_NOTONCHANNEL
    ///       "<channel> :You're not on that channel"
    ///
    ///  - Returned by the server whenever a client tries to
    ///    perform a channel affecting command for which the
    ///    client isn't a member.
    ///
    ERR_NOTONCHANNEL(CS<'a>),

    /// 443    ERR_USERONCHANNEL
    ///       "<user> <channel> :is already on channel"
    ///
    ///  - Returned when a client tries to invite a user to a
    ///    channel they are already on.
    ///
    ERR_USERONCHANNEL(CS<'a>),

    /// 444    ERR_NOLOGIN
    ///       "<user> :User not logged in"
    ///
    ///  - Returned by the summon after a SUMMON command for a
    ///    user was unable to be performed since they were not
    ///    logged in.
    ///
    ERR_NOLOGIN(CS<'a>),

    /// 445    ERR_SUMMONDISABLED
    ///       ":SUMMON has been disabled"
    ///
    ///  - Returned as a response to the SUMMON command.  MUST be
    ///    returned by any server which doesn't implement it.
    ///
    ERR_SUMMONDISABLED(CS<'a>),

    /// 446    ERR_USERSDISABLED
    ///       ":USERS has been disabled"
    ///
    ///  - Returned as a response to the USERS command.  MUST be
    ///    returned by any server which does not implement it.
    ///
    ERR_USERSDISABLED(CS<'a>),

    /// 451    ERR_NOTREGISTERED
    ///       ":You have not registered"
    ///
    ///  - Returned by the server to indicate that the client
    ///    MUST be registered before the server will allow it
    ///    to be parsed in detail.
    ///
    ERR_NOTREGISTERED(CS<'a>),

    /// 461    ERR_NEEDMOREPARAMS
    ///       "<command> :Not enough parameters"
    ///
    ///  - Returned by the server by numerous commands to
    ///    indicate to the client that it didn't supply enough
    ///    parameters.
    ///
    ERR_NEEDMOREPARAMS(CS<'a>),

    /// 462    ERR_ALREADYREGISTRED
    ///       ":Unauthorized command (already registered)"
    ///
    ///  - Returned by the server to any link which tries to
    ///    change part of the registered details (such as
    ///    password or user details from second USER message).
    ///
    ERR_ALREADYREGISTRED(CS<'a>),

    /// 463    ERR_NOPERMFORHOST
    ///       ":Your host isn't among the privileged"
    ///
    ///  - Returned to a client which attempts to register with
    ///    a server which does not been setup to allow
    ///    connections from the host the attempted connection
    ///    is tried.
    ///
    ERR_NOPERMFORHOST(CS<'a>),

    /// 464    ERR_PASSWDMISMATCH
    ///       ":Password incorrect"
    ///
    ///  - Returned to indicate a failed attempt at registering
    ///    a connection for which a password was required and
    ///    was either not given or incorrect.
    ///
    ERR_PASSWDMISMATCH(CS<'a>),

    /// 465    ERR_YOUREBANNEDCREEP
    ///       ":You are banned from this server"
    ///
    ///  - Returned after an attempt to connect and register
    ///    yourself with a server which has been setup to
    ///    explicitly deny connections to you.
    ///
    ERR_YOUREBANNEDCREEP(CS<'a>),

    /// 466    ERR_YOUWILLBEBANNED
    ///
    ///  - Sent by a server to a user to inform that access to the
    ///    server will soon be denied.
    ///
    ERR_YOUWILLBEBANNED(CS<'a>),

    /// 467    ERR_KEYSET
    ///       "<channel> :Channel key already set"
    ERR_KEYSET(CS<'a>),

    /// 471    ERR_CHANNELISFULL
    ///       "<channel> :Cannot join channel (+l)"
    ERR_CHANNELISFULL(CS<'a>),

    /// 472    ERR_UNKNOWNMODE
    ///       "<char> :is unknown mode char to me for <channel>"
    ERR_UNKNOWNMODE(CS<'a>),

    /// 473    ERR_INVITEONLYCHAN
    ///       "<channel> :Cannot join channel (+i)"
    ERR_INVITEONLYCHAN(CS<'a>),

    /// 474    ERR_BANNEDFROMCHAN
    ///       "<channel> :Cannot join channel (+b)"
    ERR_BANNEDFROMCHAN(CS<'a>),

    /// 475    ERR_BADCHANNELKEY
    ///       "<channel> :Cannot join channel (+k)"
    ERR_BADCHANNELKEY(CS<'a>),

    /// 476    ERR_BADCHANMASK
    ///       "<channel> :Bad Channel Mask"
    ERR_BADCHANMASK(CS<'a>),

    /// 477    ERR_NOCHANMODES
    ///       "<channel> :Channel doesn't support modes"
    ERR_NOCHANMODES(CS<'a>),

    /// 478    ERR_BANLISTFULL
    ///       "<channel> <char> :Channel list is full"
    ///
    ERR_BANLISTFULL(CS<'a>),

    /// 481    ERR_NOPRIVILEGES
    ///       ":Permission Denied- You're not an IRC operator"
    ///
    ///  - Any command requiring operator privileges to operate
    ///    MUST return this error to indicate the attempt was
    ///    unsuccessful.
    ///
    ERR_NOPRIVILEGES(CS<'a>),

    /// 482    ERR_CHANOPRIVSNEEDED
    ///       "<channel> :You're not channel operator"
    ///
    ///  - Any command requiring 'chanop' privileges (such as
    ///    MODE messages) MUST return this error if the client
    ///    making the attempt is not a chanop on the specified
    ///    channel.
    ///
    ERR_CHANOPRIVSNEEDED(CS<'a>),

    /// 483    ERR_CANTKILLSERVER
    ///       ":You can't kill a server!"
    ///
    ///  - Any attempts to use the KILL command on a server
    ///    are to be refused and this error returned directly
    ///    to the client.
    ///
    ERR_CANTKILLSERVER(CS<'a>),

    /// 484    ERR_RESTRICTED
    ///       ":Your connection is restricted!"
    ///
    ///  - Sent by the server to a user upon connection to indicate
    ///    the restricted nature of the connection (user mode "+r").
    ///
    ERR_RESTRICTED(CS<'a>),

    /// 485    ERR_UNIQOPPRIVSNEEDED
    ///       ":You're not the original channel operator"
    ///
    ///  - Any MODE requiring "channel creator" privileges MUST
    ///    return this error if the client making the attempt is not
    ///    a chanop on the specified channel.
    ///
    ERR_UNIQOPPRIVSNEEDED(CS<'a>),

    /// 491    ERR_NOOPERHOST
    ///       ":No O-lines for your host"
    ///
    ///  - If a client sends an OPER message and the server has
    ///    not been configured to allow connections from the
    ///    client's host as an operator, this error MUST be
    ///    returned.
    ///
    ERR_NOOPERHOST(CS<'a>),

    /// 501    ERR_UMODEUNKNOWNFLAG
    ///       ":Unknown MODE flag"
    ///
    ///  - Returned by the server to indicate that a MODE
    ///    message was sent with a nickname parameter and that
    ///    the a mode flag sent was not recognized.
    ///
    ERR_UMODEUNKNOWNFLAG(CS<'a>),

    /// 502    ERR_USERSDONTMATCH
    ///       ":Cannot change mode for other users"
    ///
    ///  - Error sent to any user trying to view or change the
    ///    user mode for a user other than themselves.
    ///
    ERR_USERSDONTMATCH(CS<'a>),

}

impl<'a> Reply<'a> {
    pub fn from_message(msg: &'a Message) -> Option<Reply<'a>> {
        use self::Reply::*;
        match msg.command() {
            "001" => msg.elements().last().map(|&e| RPL_WELCOME(Borrowed(e))),
            /*"002" => Ok(RPL_YOURHOST),
            "003" => Ok(RPL_CREATED),
            "004" => Ok(RPL_MYINFO),
            "005" => Ok(RPL_BOUNCE),
            "302" => Ok(RPL_USERHOST),
            "303" => Ok(RPL_ISON),
            "301" => Ok(RPL_AWAY),
            "305" => Ok(RPL_UNAWAY),
            "306" => Ok(RPL_NOWAWAY),
            "311" => Ok(RPL_WHOISUSER),
            "312" => Ok(RPL_WHOISSERVER),
            "313" => Ok(RPL_WHOISOPERATOR),
            "317" => Ok(RPL_WHOISIDLE),
            "318" => Ok(RPL_ENDOFWHOIS),
            "319" => Ok(RPL_WHOISCHANNELS),
            "314" => Ok(RPL_WHOWASUSER),
            "369" => Ok(RPL_ENDOFWHOWAS),
            "321" => Ok(RPL_LISTSTART),
            "322" => Ok(RPL_LIST),
            "323" => Ok(RPL_LISTEND),
            "325" => Ok(RPL_UNIQOPIS),
            "324" => Ok(RPL_CHANNELMODEIS),
            "331" => Ok(RPL_NOTOPIC),
            "332" => Ok(RPL_TOPIC),
            "341" => Ok(RPL_INVITING),
            "342" => Ok(RPL_SUMMONING),
            "346" => Ok(RPL_INVITELIST),
            "347" => Ok(RPL_ENDOFINVITELIST),
            "348" => Ok(RPL_EXCEPTLIST),
            "349" => Ok(RPL_ENDOFEXCEPTLIST),
            "351" => Ok(RPL_VERSION),
            "352" => Ok(RPL_WHOREPLY),
            "315" => Ok(RPL_ENDOFWHO),
            "353" => Ok(RPL_NAMREPLY),
            "366" => Ok(RPL_ENDOFNAMES),
            "364" => Ok(RPL_LINKS),
            "365" => Ok(RPL_ENDOFLINKS),
            "367" => Ok(RPL_BANLIST),
            "368" => Ok(RPL_ENDOFBANLIST),
            "371" => Ok(RPL_INFO),
            "374" => Ok(RPL_ENDOFINFO),
            "375" => Ok(RPL_MOTDSTART),
            "372" => Ok(RPL_MOTD),
            "376" => Ok(RPL_ENDOFMOTD),
            "381" => Ok(RPL_YOUREOPER),
            "382" => Ok(RPL_REHASHING),
            "383" => Ok(RPL_YOURESERVICE),
            "391" => Ok(RPL_TIME),
            "392" => Ok(RPL_USERSSTART),
            "393" => Ok(RPL_USERS),
            "394" => Ok(RPL_ENDOFUSERS),
            "395" => Ok(RPL_NOUSERS),
            "200" => Ok(RPL_TRACELINK),
            "201" => Ok(RPL_TRACECONNECTING),
            "202" => Ok(RPL_TRACEHANDSHAKE),
            "203" => Ok(RPL_TRACEUNKNOWN),
            "204" => Ok(RPL_TRACEOPERATOR),
            "205" => Ok(RPL_TRACEUSER),
            "206" => Ok(RPL_TRACESERVER),
            "207" => Ok(RPL_TRACESERVICE),
            "208" => Ok(RPL_TRACENEWTYPE),
            "209" => Ok(RPL_TRACECLASS),
            "210" => Ok(RPL_TRACERECONNECT),
            "261" => Ok(RPL_TRACELOG),
            "262" => Ok(RPL_TRACEEND),
            "211" => Ok(RPL_STATSLINKINFO),
            "212" => Ok(RPL_STATSCOMMANDS),
            "219" => Ok(RPL_ENDOFSTATS),
            "242" => Ok(RPL_STATSUPTIME),
            "243" => Ok(RPL_STATSOLINE),
            "221" => Ok(RPL_UMODEIS),
            "234" => Ok(RPL_SERVLIST),
            "235" => Ok(RPL_SERVLISTEND),
            "251" => Ok(RPL_LUSERCLIENT),
            "252" => Ok(RPL_LUSEROP),
            "253" => Ok(RPL_LUSERUNKNOWN),
            "254" => Ok(RPL_LUSERCHANNELS),
            "255" => Ok(RPL_LUSERME),
            "256" => Ok(RPL_ADMINME),
            "257" => Ok(RPL_ADMINLOC1),
            "258" => Ok(RPL_ADMINLOC2),
            "259" => Ok(RPL_ADMINEMAIL),
            "263" => Ok(RPL_TRYAGAIN),
            "401" => Ok(ERR_NOSUCHNICK),
            "402" => Ok(ERR_NOSUCHSERVER),
            "403" => Ok(ERR_NOSUCHCHANNEL),
            "404" => Ok(ERR_CANNOTSENDTOCHAN),
            "405" => Ok(ERR_TOOMANYCHANNELS),
            "406" => Ok(ERR_WASNOSUCHNICK),
            "407" => Ok(ERR_TOOMANYTARGETS),
            "408" => Ok(ERR_NOSUCHSERVICE),
            "409" => Ok(ERR_NOORIGIN),
            "411" => Ok(ERR_NORECIPIENT),
            "412" => Ok(ERR_NOTEXTTOSEND),
            "413" => Ok(ERR_NOTOPLEVEL),
            "414" => Ok(ERR_WILDTOPLEVEL),
            "415" => Ok(ERR_BADMASK),
            "421" => Ok(ERR_UNKNOWNCOMMAND),
            "422" => Ok(ERR_NOMOTD),
            "423" => Ok(ERR_NOADMININFO),
            "424" => Ok(ERR_FILEERROR),
            "431" => Ok(ERR_NONICKNAMEGIVEN),
            "432" => Ok(ERR_ERRONEUSNICKNAME),
            "433" => Ok(ERR_NICKNAMEINUSE),
            "436" => Ok(ERR_NICKCOLLISION),
            "437" => Ok(ERR_UNAVAILRESOURCE),
            "441" => Ok(ERR_USERNOTINCHANNEL),
            "442" => Ok(ERR_NOTONCHANNEL),
            "443" => Ok(ERR_USERONCHANNEL),
            "444" => Ok(ERR_NOLOGIN),
            "445" => Ok(ERR_SUMMONDISABLED),
            "446" => Ok(ERR_USERSDISABLED),
            "451" => Ok(ERR_NOTREGISTERED),
            "461" => Ok(ERR_NEEDMOREPARAMS),
            "462" => Ok(ERR_ALREADYREGISTRED),
            "463" => Ok(ERR_NOPERMFORHOST),
            "464" => Ok(ERR_PASSWDMISMATCH),
            "465" => Ok(ERR_YOUREBANNEDCREEP),
            "466" => Ok(ERR_YOUWILLBEBANNED),
            "467" => Ok(ERR_KEYSET),
            "471" => Ok(ERR_CHANNELISFULL),
            "472" => Ok(ERR_UNKNOWNMODE),
            "473" => Ok(ERR_INVITEONLYCHAN),
            "474" => Ok(ERR_BANNEDFROMCHAN),
            "475" => Ok(ERR_BADCHANNELKEY),
            "476" => Ok(ERR_BADCHANMASK),
            "477" => Ok(ERR_NOCHANMODES),
            "478" => Ok(ERR_BANLISTFULL),
            "481" => Ok(ERR_NOPRIVILEGES),
            "482" => Ok(ERR_CHANOPRIVSNEEDED),
            "483" => Ok(ERR_CANTKILLSERVER),
            "484" => Ok(ERR_RESTRICTED),
            "485" => Ok(ERR_UNIQOPPRIVSNEEDED),
            "491" => Ok(ERR_NOOPERHOST),
            "501" => Ok(ERR_UMODEUNKNOWNFLAG),
            "502" => Ok(ERR_USERSDONTMATCH),*/
            _ => None
        }
     }

    pub fn to_message(&'a self) -> Message {
        use self::Reply::*;
        match self {
            &RPL_WELCOME(ref s) => Message::format(None, Borrowed("001"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_YOURHOST(ref s) => Message::format(None, Borrowed("002"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_CREATED(ref s) => Message::format(None, Borrowed("003"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_MYINFO(ref s) => Message::format(None, Borrowed("004"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_BOUNCE(ref s) => Message::format(None, Borrowed("005"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_USERHOST(ref s) => Message::format(None, Borrowed("302"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ISON(ref s) => Message::format(None, Borrowed("303"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_AWAY(ref s) => Message::format(None, Borrowed("301"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_UNAWAY(ref s) => Message::format(None, Borrowed("305"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_NOWAWAY(ref s) => Message::format(None, Borrowed("306"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOISUSER(ref s) => Message::format(None, Borrowed("311"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOISSERVER(ref s) => Message::format(None, Borrowed("312"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOISOPERATOR(ref s) => Message::format(None, Borrowed("313"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOISIDLE(ref s) => Message::format(None, Borrowed("317"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFWHOIS(ref s) => Message::format(None, Borrowed("318"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOISCHANNELS(ref s) => Message::format(None, Borrowed("319"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOWASUSER(ref s) => Message::format(None, Borrowed("314"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFWHOWAS(ref s) => Message::format(None, Borrowed("369"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LISTSTART => Message::format(None, Borrowed("321"), vec![], None, MsgType::Irc),
            &RPL_LIST(ref s) => Message::format(None, Borrowed("322"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LISTEND(ref s) => Message::format(None, Borrowed("323"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_UNIQOPIS(ref s) => Message::format(None, Borrowed("325"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_CHANNELMODEIS(ref s) => Message::format(None, Borrowed("324"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_NOTOPIC(ref s) => Message::format(None, Borrowed("331"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TOPIC(ref s) => Message::format(None, Borrowed("332"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_INVITING(ref s) => Message::format(None, Borrowed("341"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_SUMMONING(ref s) => Message::format(None, Borrowed("342"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_INVITELIST(ref s) => Message::format(None, Borrowed("346"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFINVITELIST(ref s) => Message::format(None, Borrowed("347"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_EXCEPTLIST(ref s) => Message::format(None, Borrowed("348"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFEXCEPTLIST(ref s) => Message::format(None, Borrowed("349"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_VERSION(ref s) => Message::format(None, Borrowed("351"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_WHOREPLY(ref s) => Message::format(None, Borrowed("352"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFWHO(ref s) => Message::format(None, Borrowed("315"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_NAMREPLY(ref s) => Message::format(None, Borrowed("353"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFNAMES(ref s) => Message::format(None, Borrowed("366"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LINKS(ref s) => Message::format(None, Borrowed("364"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFLINKS(ref s) => Message::format(None, Borrowed("365"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_BANLIST(ref s) => Message::format(None, Borrowed("367"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFBANLIST(ref s) => Message::format(None, Borrowed("368"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_INFO(ref s) => Message::format(None, Borrowed("371"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFINFO(ref s) => Message::format(None, Borrowed("374"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_MOTDSTART(ref s) => Message::format(None, Borrowed("375"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_MOTD(ref s) => Message::format(None, Borrowed("372"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFMOTD(ref s) => Message::format(None, Borrowed("376"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_YOUREOPER(ref s) => Message::format(None, Borrowed("381"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_REHASHING(ref s) => Message::format(None, Borrowed("382"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_YOURESERVICE(ref s) => Message::format(None, Borrowed("383"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TIME(ref s) => Message::format(None, Borrowed("391"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_USERSSTART(ref s) => Message::format(None, Borrowed("392"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_USERS(ref s) => Message::format(None, Borrowed("393"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFUSERS(ref s) => Message::format(None, Borrowed("394"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_NOUSERS(ref s) => Message::format(None, Borrowed("395"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACELINK(ref s) => Message::format(None, Borrowed("200"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACECONNECTING(ref s) => Message::format(None, Borrowed("201"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACEHANDSHAKE(ref s) => Message::format(None, Borrowed("202"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACEUNKNOWN(ref s) => Message::format(None, Borrowed("203"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACEOPERATOR(ref s) => Message::format(None, Borrowed("204"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACEUSER(ref s) => Message::format(None, Borrowed("205"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACESERVER(ref s) => Message::format(None, Borrowed("206"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACESERVICE(ref s) => Message::format(None, Borrowed("207"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACENEWTYPE(ref s) => Message::format(None, Borrowed("208"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACECLASS(ref s) => Message::format(None, Borrowed("209"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACERECONNECT(ref s) => Message::format(None, Borrowed("210"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACELOG(ref s) => Message::format(None, Borrowed("261"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRACEEND(ref s) => Message::format(None, Borrowed("262"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_STATSLINKINFO(ref s) => Message::format(None, Borrowed("211"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_STATSCOMMANDS(ref s) => Message::format(None, Borrowed("212"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ENDOFSTATS(ref s) => Message::format(None, Borrowed("219"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_STATSUPTIME(ref s) => Message::format(None, Borrowed("242"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_STATSOLINE(ref s) => Message::format(None, Borrowed("243"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_UMODEIS(ref s) => Message::format(None, Borrowed("221"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_SERVLIST(ref s) => Message::format(None, Borrowed("234"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_SERVLISTEND(ref s) => Message::format(None, Borrowed("235"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LUSERCLIENT(ref s) => Message::format(None, Borrowed("251"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LUSEROP(ref s) => Message::format(None, Borrowed("252"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LUSERUNKNOWN(ref s) => Message::format(None, Borrowed("253"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LUSERCHANNELS(ref s) => Message::format(None, Borrowed("254"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_LUSERME(ref s) => Message::format(None, Borrowed("255"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ADMINME(ref s) => Message::format(None, Borrowed("256"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ADMINLOC1(ref s) => Message::format(None, Borrowed("257"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ADMINLOC2(ref s) => Message::format(None, Borrowed("258"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_ADMINEMAIL(ref s) => Message::format(None, Borrowed("259"), vec![], Some(s.clone()), MsgType::Irc),
            &RPL_TRYAGAIN(ref s) => Message::format(None, Borrowed("263"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOSUCHNICK(ref s) => Message::format(None, Borrowed("401"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOSUCHSERVER(ref s) => Message::format(None, Borrowed("402"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOSUCHCHANNEL(ref s) => Message::format(None, Borrowed("403"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_CANNOTSENDTOCHAN(ref s) => Message::format(None, Borrowed("404"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_TOOMANYCHANNELS(ref s) => Message::format(None, Borrowed("405"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_WASNOSUCHNICK(ref s) => Message::format(None, Borrowed("406"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_TOOMANYTARGETS(ref s) => Message::format(None, Borrowed("407"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOSUCHSERVICE(ref s) => Message::format(None, Borrowed("408"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOORIGIN(ref s) => Message::format(None, Borrowed("409"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NORECIPIENT(ref s) => Message::format(None, Borrowed("411"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOTEXTTOSEND(ref s) => Message::format(None, Borrowed("412"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOTOPLEVEL(ref s) => Message::format(None, Borrowed("413"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_WILDTOPLEVEL(ref s) => Message::format(None, Borrowed("414"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_BADMASK(ref s) => Message::format(None, Borrowed("415"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_UNKNOWNCOMMAND(ref s) => Message::format(None, Borrowed("421"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOMOTD(ref s) => Message::format(None, Borrowed("422"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOADMININFO(ref s) => Message::format(None, Borrowed("423"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_FILEERROR(ref s) => Message::format(None, Borrowed("424"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NONICKNAMEGIVEN(ref s) => Message::format(None, Borrowed("431"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_ERRONEUSNICKNAME(ref s) => Message::format(None, Borrowed("432"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NICKNAMEINUSE(ref s) => Message::format(None, Borrowed("433"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NICKCOLLISION(ref s) => Message::format(None, Borrowed("436"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_UNAVAILRESOURCE(ref s) => Message::format(None, Borrowed("437"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_USERNOTINCHANNEL(ref s) => Message::format(None, Borrowed("441"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOTONCHANNEL(ref s) => Message::format(None, Borrowed("442"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_USERONCHANNEL(ref s) => Message::format(None, Borrowed("443"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOLOGIN(ref s) => Message::format(None, Borrowed("444"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_SUMMONDISABLED(ref s) => Message::format(None, Borrowed("445"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_USERSDISABLED(ref s) => Message::format(None, Borrowed("446"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOTREGISTERED(ref s) => Message::format(None, Borrowed("451"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NEEDMOREPARAMS(ref s) => Message::format(None, Borrowed("461"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_ALREADYREGISTRED(ref s) => Message::format(None, Borrowed("462"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOPERMFORHOST(ref s) => Message::format(None, Borrowed("463"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_PASSWDMISMATCH(ref s) => Message::format(None, Borrowed("464"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_YOUREBANNEDCREEP(ref s) => Message::format(None, Borrowed("465"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_YOUWILLBEBANNED(ref s) => Message::format(None, Borrowed("466"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_KEYSET(ref s) => Message::format(None, Borrowed("467"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_CHANNELISFULL(ref s) => Message::format(None, Borrowed("471"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_UNKNOWNMODE(ref s) => Message::format(None, Borrowed("472"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_INVITEONLYCHAN(ref s) => Message::format(None, Borrowed("473"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_BANNEDFROMCHAN(ref s) => Message::format(None, Borrowed("474"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_BADCHANNELKEY(ref s) => Message::format(None, Borrowed("475"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_BADCHANMASK(ref s) => Message::format(None, Borrowed("476"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOCHANMODES(ref s) => Message::format(None, Borrowed("477"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_BANLISTFULL(ref s) => Message::format(None, Borrowed("478"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOPRIVILEGES(ref s) => Message::format(None, Borrowed("481"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_CHANOPRIVSNEEDED(ref s) => Message::format(None, Borrowed("482"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_CANTKILLSERVER(ref s) => Message::format(None, Borrowed("483"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_RESTRICTED(ref s) => Message::format(None, Borrowed("484"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_UNIQOPPRIVSNEEDED(ref s) => Message::format(None, Borrowed("485"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_NOOPERHOST(ref s) => Message::format(None, Borrowed("491"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_UMODEUNKNOWNFLAG(ref s) => Message::format(None, Borrowed("501"), vec![], Some(s.clone()), MsgType::Irc),
            &ERR_USERSDONTMATCH(ref s) => Message::format(None, Borrowed("502"), vec![], Some(s.clone()), MsgType::Irc),
        }
     }
}
