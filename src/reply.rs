use ::{ Result, IrscError };
use std::str::FromStr;
use std::borrow::ToOwned;
#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Reply {
    /// 001    RPL_WELCOME
    ///       "Welcome to the Internet Relay Network
    ///        <nick>!<user>@<host>"
    RPL_WELCOME = 001,

    /// 002    RPL_YOURHOST
    ///       "Your host is <servername>, running version <ver>"
    RPL_YOURHOST = 002,

    /// 003    RPL_CREATED
    ///       "This server was created <date>"
    RPL_CREATED = 003,

    /// 004    RPL_MYINFO
    ///       "<servername> <version> <available user modes>
    ///        <available channel modes>"
    /// 
    ///  - The server sends Replies 001 to 004 to a user upon
    ///    successful registration.
    /// 
    RPL_MYINFO = 004,

    /// 005    RPL_BOUNCE
    ///       "Try server <server name>, port <port number>"
    /// 
    ///  - Sent by the server to a user to suggest an alternative
    ///    server.  This is often used when the connection is
    ///    refused because the server is already full.
    /// 
    RPL_BOUNCE = 005,

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
    RPL_USERHOST = 302,

    /// 303    RPL_ISON
    ///       ":*1<nick> *( " " <nick> )"
    /// 
    ///  - Reply format used by ISON to list replies to the
    ///    query list.
    /// 
    RPL_ISON = 303,

    /// 301    RPL_AWAY
    ///       "<nick> :<away message>"
    RPL_AWAY = 301,

    /// 305    RPL_UNAWAY
    ///       ":You are no longer marked as being away"
    RPL_UNAWAY = 305,

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
    RPL_NOWAWAY = 306,

    /// 311    RPL_WHOISUSER
    ///       "<nick> <user> <host> * :<real name>"
    RPL_WHOISUSER = 311,

    /// 312    RPL_WHOISSERVER
    ///       "<nick> <server> :<server info>"
    RPL_WHOISSERVER = 312,

    /// 313    RPL_WHOISOPERATOR
    ///       "<nick> :is an IRC operator"
    RPL_WHOISOPERATOR = 313,

    /// 317    RPL_WHOISIDLE
    ///       "<nick> <integer> :seconds idle"
    RPL_WHOISIDLE = 317,

    /// 318    RPL_ENDOFWHOIS
    ///       "<nick> :End of WHOIS list"
    RPL_ENDOFWHOIS = 318,

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
    RPL_WHOISCHANNELS = 319,

    /// 314    RPL_WHOWASUSER
    ///       "<nick> <user> <host> * :<real name>"
    RPL_WHOWASUSER = 314,

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
    RPL_ENDOFWHOWAS = 369,

    /// 321    RPL_LISTSTART
    ///       Obsolete. Not used.
    /// 
    RPL_LISTSTART = 321,

    /// 322    RPL_LIST
    ///       "<channel> <# visible> :<topic>"
    RPL_LIST = 322,

    /// 323    RPL_LISTEND
    ///       ":End of LIST"
    /// 
    ///  - Replies RPL_LIST, RPL_LISTEND mark the actual replies
    ///    with data and end of the server's response to a LIST
    ///    command.  If there are no channels available to return,
    ///    only the end reply MUST be sent.
    /// 
    RPL_LISTEND = 323,

    /// 325    RPL_UNIQOPIS
    ///       "<channel> <nickname>"
    /// 
    RPL_UNIQOPIS = 325,

    /// 324    RPL_CHANNELMODEIS
    ///       "<channel> <mode> <mode params>"
    /// 
    RPL_CHANNELMODEIS = 324,

    /// 331    RPL_NOTOPIC
    ///       "<channel> :No topic is set"
    RPL_NOTOPIC = 331,

    /// 332    RPL_TOPIC
    ///       "<channel> :<topic>"
    /// 
    ///  - When sending a TOPIC message to determine the
    ///    channel topic, one of two replies is sent.  If
    ///    the topic is set, RPL_TOPIC is sent back else
    ///    RPL_NOTOPIC.
    /// 
    RPL_TOPIC = 332,

    /// 341    RPL_INVITING
    ///       "<channel> <nick>"
    /// 
    ///  - Returned by the server to indicate that the
    ///    attempted INVITE message was successful and is
    ///    being passed onto the end client.
    /// 
    RPL_INVITING = 341,

    /// 342    RPL_SUMMONING
    ///       "<user> :Summoning user to IRC"
    /// 
    ///  - Returned by a server answering a SUMMON message to
    ///    indicate that it is summoning that user.
    /// 
    RPL_SUMMONING = 342,

    /// 346    RPL_INVITELIST
    ///       "<channel> <invitemask>"
    RPL_INVITELIST = 346,

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
    RPL_ENDOFINVITELIST = 347,

    /// 348    RPL_EXCEPTLIST
    ///       "<channel> <exceptionmask>"
    RPL_EXCEPTLIST = 348,

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
    RPL_ENDOFEXCEPTLIST = 349,

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
    RPL_VERSION = 351,

    /// 352    RPL_WHOREPLY
    ///       "<channel> <user> <host> <server> <nick>
    ///       ( "H" / "G" > ["*"] [ ( "@" / "+" ) ]
    ///       :<hopcount> <real name>"
    /// 
    RPL_WHOREPLY = 352,

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
    RPL_ENDOFWHO = 315,

    /// 353    RPL_NAMREPLY
    ///       "( "=" / "*" / "@" ) <channel>
    ///        :[ "@" / "+" ] <nick> *( " " [ "@" / "+" ] <nick> )
    ///  - "@" is used for secret channels, "*" for private
    ///    channels, and "=" for others (public channels).
    /// 
    RPL_NAMREPLY = 353,

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
    RPL_ENDOFNAMES = 366,

    /// 364    RPL_LINKS
    ///       "<mask> <server> :<hopcount> <server info>"
    RPL_LINKS = 364,

    /// 365    RPL_ENDOFLINKS
    ///       "<mask> :End of LINKS list"
    /// 
    ///  - In replying to the LINKS message, a server MUST send
    ///    replies back using the RPL_LINKS numeric and mark the
    ///    end of the list using an RPL_ENDOFLINKS reply.
    /// 
    RPL_ENDOFLINKS = 365,

    /// 367    RPL_BANLIST
    ///       "<channel> <banmask>"
    RPL_BANLIST = 367,

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
    RPL_ENDOFBANLIST = 368,

    /// 371    RPL_INFO
    ///       ":<string>"
    RPL_INFO = 371,

    /// 374    RPL_ENDOFINFO
    ///       ":End of INFO list"
    /// 
    ///  - A server responding to an INFO message is required to
    ///    send all its 'info' in a series of RPL_INFO messages
    ///    with a RPL_ENDOFINFO reply to indicate the end of the
    ///    replies.
    /// 
    RPL_ENDOFINFO = 374,

    /// 375    RPL_MOTDSTART
    ///       ":- <server> Message of the day - "
    RPL_MOTDSTART = 375,

    /// 372    RPL_MOTD
    ///       ":- <text>"
    RPL_MOTD = 372,

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
    RPL_ENDOFMOTD = 376,

    /// 381    RPL_YOUREOPER
    ///       ":You are now an IRC operator"
    /// 
    ///  - RPL_YOUREOPER is sent back to a client which has
    ///    just successfully issued an OPER message and gained
    ///    operator status.
    /// 
    RPL_YOUREOPER = 381,

    /// 382    RPL_REHASHING
    ///       "<config file> :Rehashing"
    /// 
    ///  - If the REHASH option is used and an operator sends
    ///    a REHASH message, an RPL_REHASHING is sent back to
    ///    the operator.
    /// 
    RPL_REHASHING = 382,

    /// 383    RPL_YOURESERVICE
    ///       "You are service <servicename>"
    /// 
    ///  - Sent by the server to a service upon successful
    ///    registration.
    /// 
    RPL_YOURESERVICE = 383,

    /// 391    RPL_TIME
    ///       "<server> :<string showing server's local time>"
    /// 
    ///  - When replying to the TIME message, a server MUST send
    ///    the reply using the RPL_TIME format above.  The string
    ///    showing the time need only contain the correct day and
    ///    time there.  There is no further requirement for the
    ///    time string.
    /// 
    RPL_TIME = 391,

    /// 392    RPL_USERSSTART
    ///       ":UserID   Terminal  Host"
    RPL_USERSSTART = 392,

    /// 393    RPL_USERS
    ///       ":<username> <ttyline> <hostname>"
    RPL_USERS = 393,

    /// 394    RPL_ENDOFUSERS
    ///       ":End of users"
    RPL_ENDOFUSERS = 394,

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
    RPL_NOUSERS = 395,

    /// 200    RPL_TRACELINK
    ///       "Link <version & debug level> <destination>
    ///        <next server> V<protocol version>
    ///        <link uptime in seconds> <backstream sendq>
    ///        <upstream sendq>"
    RPL_TRACELINK = 200,

    /// 201    RPL_TRACECONNECTING
    ///       "Try. <class> <server>"
    RPL_TRACECONNECTING = 201,

    /// 202    RPL_TRACEHANDSHAKE
    ///       "H.S. <class> <server>"
    RPL_TRACEHANDSHAKE = 202,

    /// 203    RPL_TRACEUNKNOWN
    ///       "???? <class> [<client IP address in dot form>]"
    RPL_TRACEUNKNOWN = 203,

    /// 204    RPL_TRACEOPERATOR
    ///       "Oper <class> <nick>"
    RPL_TRACEOPERATOR = 204,

    /// 205    RPL_TRACEUSER
    ///       "User <class> <nick>"
    RPL_TRACEUSER = 205,

    /// 206    RPL_TRACESERVER
    ///       "Serv <class> <int>S <int>C <server>
    ///        <nick!user|*!*>@<host|server> V<protocol version>"
    RPL_TRACESERVER = 206,

    /// 207    RPL_TRACESERVICE
    ///       "Service <class> <name> <type> <active type>"
    RPL_TRACESERVICE = 207,

    /// 208    RPL_TRACENEWTYPE
    ///       "<newtype> 0 <client name>"
    RPL_TRACENEWTYPE = 208,

    /// 209    RPL_TRACECLASS
    ///       "Class <class> <count>"
    RPL_TRACECLASS = 209,

    /// 210    RPL_TRACERECONNECT
    ///       Unused.
    RPL_TRACERECONNECT = 210,

    /// 261    RPL_TRACELOG
    ///       "File <logfile> <debug level>"
    RPL_TRACELOG = 261,

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
    RPL_TRACEEND = 262,

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
    RPL_STATSLINKINFO = 211,

    /// 212    RPL_STATSCOMMANDS
    ///       "<command> <count> <byte count> <remote count>"
    /// 
    ///  - reports statistics on commands usage.
    /// 
    RPL_STATSCOMMANDS = 212,

    /// 219    RPL_ENDOFSTATS
    ///       "<stats letter> :End of STATS report"
    /// 
    RPL_ENDOFSTATS = 219,

    /// 242    RPL_STATSUPTIME
    ///       ":Server Up %d days %d:%02d:%02d"
    /// 
    ///  - reports the server uptime.
    /// 
    RPL_STATSUPTIME = 242,

    /// 243    RPL_STATSOLINE
    ///       "O <hostmask> * <name>"
    /// 
    ///  - reports the allowed hosts from where user may become IRC
    ///    operators.
    /// 
    RPL_STATSOLINE = 243,

    /// 221    RPL_UMODEIS
    ///       "<user mode string>"
    /// 
    ///  - To answer a query about a client's own mode,
    ///    RPL_UMODEIS is sent back.
    /// 
    RPL_UMODEIS = 221,

    /// 234    RPL_SERVLIST
    ///       "<name> <server> <mask> <type> <hopcount> <info>"
    /// 
    RPL_SERVLIST = 234,

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
    RPL_SERVLISTEND = 235,

    /// 251    RPL_LUSERCLIENT
    ///       ":There are <integer> users and <integer>
    ///        services on <integer> servers"
    RPL_LUSERCLIENT = 251,

    /// 252    RPL_LUSEROP
    ///       "<integer> :operator(s) online"
    RPL_LUSEROP = 252,

    /// 253    RPL_LUSERUNKNOWN
    ///       "<integer> :unknown connection(s)"
    RPL_LUSERUNKNOWN = 253,

    /// 254    RPL_LUSERCHANNELS
    ///       "<integer> :channels formed"
    RPL_LUSERCHANNELS = 254,

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
    RPL_LUSERME = 255,

    /// 256    RPL_ADMINME
    ///       "<server> :Administrative info"
    RPL_ADMINME = 256,

    /// 257    RPL_ADMINLOC1
    ///       ":<admin info>"
    RPL_ADMINLOC1 = 257,

    /// 258    RPL_ADMINLOC2
    ///       ":<admin info>"
    RPL_ADMINLOC2 = 258,

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
    RPL_ADMINEMAIL = 259,

    /// 263    RPL_TRYAGAIN
    ///       "<command> :Please wait a while and try again."
    /// 
    ///  - When a server drops a command without processing it,
    ///    it MUST use the reply RPL_TRYAGAIN to inform the
    ///    originating client.
    /// 
    RPL_TRYAGAIN = 263,

    /// 401    ERR_NOSUCHNICK
    ///       "<nickname> :No such nick/channel"
    /// 
    ///   - Used to indicate the nickname parameter supplied to a
    ///     command is currently unused.
    /// 
    ERR_NOSUCHNICK = 401,

    /// 402    ERR_NOSUCHSERVER
    ///       "<server name> :No such server"
    /// 
    ///  - Used to indicate the server name given currently
    ///    does not exist.
    /// 
    ERR_NOSUCHSERVER = 402,

    /// 403    ERR_NOSUCHCHANNEL
    ///       "<channel name> :No such channel"
    /// 
    ///  - Used to indicate the given channel name is invalid.
    /// 
    ERR_NOSUCHCHANNEL = 403,

    /// 404    ERR_CANNOTSENDTOCHAN
    ///       "<channel name> :Cannot send to channel"
    /// 
    ///  - Sent to a user who is either (a) not on a channel
    ///    which is mode +n or (b) not a chanop (or mode +v) on
    ///    a channel which has mode +m set or where the user is
    ///    banned and is trying to send a PRIVMSG message to
    ///    that channel.
    /// 
    ERR_CANNOTSENDTOCHAN = 404,

    /// 405    ERR_TOOMANYCHANNELS
    ///       "<channel name> :You have joined too many channels"
    /// 
    ///  - Sent to a user when they have joined the maximum
    ///    number of allowed channels and they try to join
    ///    another channel.
    /// 
    ERR_TOOMANYCHANNELS = 405,

    /// 406    ERR_WASNOSUCHNICK
    ///       "<nickname> :There was no such nickname"
    /// 
    ///  - Returned by WHOWAS to indicate there is no history
    ///    information for that nickname.
    /// 
    ERR_WASNOSUCHNICK = 406,

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
    ERR_TOOMANYTARGETS = 407,

    /// 408    ERR_NOSUCHSERVICE
    ///       "<service name> :No such service"
    /// 
    ///  - Returned to a client which is attempting to send a SQUERY
    ///    to a service which does not exist.
    /// 
    ERR_NOSUCHSERVICE = 408,

    /// 409    ERR_NOORIGIN
    ///       ":No origin specified"
    /// 
    ///  - PING or PONG message missing the originator parameter.
    /// 
    ERR_NOORIGIN = 409,

    /// 411    ERR_NORECIPIENT
    ///       ":No recipient given (<command>)"
    ERR_NORECIPIENT = 411,

    /// 412    ERR_NOTEXTTOSEND
    ///       ":No text to send"
    ERR_NOTEXTTOSEND = 412,

    /// 413    ERR_NOTOPLEVEL
    ///       "<mask> :No toplevel domain specified"
    ERR_NOTOPLEVEL = 413,

    /// 414    ERR_WILDTOPLEVEL
    ///       "<mask> :Wildcard in toplevel domain"
    ERR_WILDTOPLEVEL = 414,

    /// 415    ERR_BADMASK
    ///       "<mask> :Bad Server/host mask"
    /// 
    ///  - 412 - 415 are returned by PRIVMSG to indicate that
    ///    the message wasn't delivered for some reason.
    ///    ERR_NOTOPLEVEL and ERR_WILDTOPLEVEL are errors that
    ///    are returned when an invalid use of
    ///    "PRIVMSG $<server>" or "PRIVMSG #<host>" is attempted.
    /// 
    ERR_BADMASK = 415,

    /// 421    ERR_UNKNOWNCOMMAND
    ///       "<command> :Unknown command"
    /// 
    ///  - Returned to a registered client to indicate that the
    ///    command sent is unknown by the server.
    /// 
    ERR_UNKNOWNCOMMAND = 421,

    /// 422    ERR_NOMOTD
    ///       ":MOTD File is missing"
    /// 
    ///  - Server's MOTD file could not be opened by the server.
    /// 
    ERR_NOMOTD = 422,

    /// 423    ERR_NOADMININFO
    ///       "<server> :No administrative info available"
    /// 
    ///  - Returned by a server in response to an ADMIN message
    ///    when there is an error in finding the appropriate
    ///    information.
    /// 
    ERR_NOADMININFO = 423,

    /// 424    ERR_FILEERROR
    ///       ":File error doing <file op> on <file>"
    /// 
    ///  - Generic error message used to report a failed file
    ///    operation during the processing of a message.
    /// 
    ERR_FILEERROR = 424,

    /// 431    ERR_NONICKNAMEGIVEN
    ///       ":No nickname given"
    /// 
    ///  - Returned when a nickname parameter expected for a
    ///    command and isn't found.
    /// 
    ERR_NONICKNAMEGIVEN = 431,

    /// 432    ERR_ERRONEUSNICKNAME
    ///       "<nick> :Erroneous nickname"
    /// 
    ///  - Returned after receiving a NICK message which contains
    ///    characters which do not fall in the defined set.  See
    ///    section 2.3.1 for details on valid nicknames.
    /// 
    ERR_ERRONEUSNICKNAME = 432,

    /// 433    ERR_NICKNAMEINUSE
    ///       "<nick> :Nickname is already in use"
    /// 
    ///  - Returned when a NICK message is processed that results
    ///    in an attempt to change to a currently existing
    ///    nickname.
    /// 
    ERR_NICKNAMEINUSE = 433,

    /// 436    ERR_NICKCOLLISION
    ///       "<nick> :Nickname collision KILL from <user>@<host>"
    /// 
    ///  - Returned by a server to a client when it detects a
    ///    nickname collision (registered of a NICK that
    ///    already exists by another server).
    /// 
    ERR_NICKCOLLISION = 436,

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
    ERR_UNAVAILRESOURCE = 437,

    /// 441    ERR_USERNOTINCHANNEL
    ///       "<nick> <channel> :They aren't on that channel"
    /// 
    ///  - Returned by the server to indicate that the target
    ///    user of the command is not on the given channel.
    /// 
    ERR_USERNOTINCHANNEL = 441,

    /// 442    ERR_NOTONCHANNEL
    ///       "<channel> :You're not on that channel"
    /// 
    ///  - Returned by the server whenever a client tries to
    ///    perform a channel affecting command for which the
    ///    client isn't a member.
    /// 
    ERR_NOTONCHANNEL = 442,

    /// 443    ERR_USERONCHANNEL
    ///       "<user> <channel> :is already on channel"
    /// 
    ///  - Returned when a client tries to invite a user to a
    ///    channel they are already on.
    /// 
    ERR_USERONCHANNEL = 443,

    /// 444    ERR_NOLOGIN
    ///       "<user> :User not logged in"
    /// 
    ///  - Returned by the summon after a SUMMON command for a
    ///    user was unable to be performed since they were not
    ///    logged in.
    /// 
    ERR_NOLOGIN = 444,

    /// 445    ERR_SUMMONDISABLED
    ///       ":SUMMON has been disabled"
    /// 
    ///  - Returned as a response to the SUMMON command.  MUST be
    ///    returned by any server which doesn't implement it.
    /// 
    ERR_SUMMONDISABLED = 445,

    /// 446    ERR_USERSDISABLED
    ///       ":USERS has been disabled"
    /// 
    ///  - Returned as a response to the USERS command.  MUST be
    ///    returned by any server which does not implement it.
    /// 
    ERR_USERSDISABLED = 446,

    /// 451    ERR_NOTREGISTERED
    ///       ":You have not registered"
    /// 
    ///  - Returned by the server to indicate that the client
    ///    MUST be registered before the server will allow it
    ///    to be parsed in detail.
    /// 
    ERR_NOTREGISTERED = 451,

    /// 461    ERR_NEEDMOREPARAMS
    ///       "<command> :Not enough parameters"
    /// 
    ///  - Returned by the server by numerous commands to
    ///    indicate to the client that it didn't supply enough
    ///    parameters.
    /// 
    ERR_NEEDMOREPARAMS = 461,

    /// 462    ERR_ALREADYREGISTRED
    ///       ":Unauthorized command (already registered)"
    /// 
    ///  - Returned by the server to any link which tries to
    ///    change part of the registered details (such as
    ///    password or user details from second USER message).
    /// 
    ERR_ALREADYREGISTRED = 462,

    /// 463    ERR_NOPERMFORHOST
    ///       ":Your host isn't among the privileged"
    /// 
    ///  - Returned to a client which attempts to register with
    ///    a server which does not been setup to allow
    ///    connections from the host the attempted connection
    ///    is tried.
    /// 
    ERR_NOPERMFORHOST = 463,

    /// 464    ERR_PASSWDMISMATCH
    ///       ":Password incorrect"
    /// 
    ///  - Returned to indicate a failed attempt at registering
    ///    a connection for which a password was required and
    ///    was either not given or incorrect.
    /// 
    ERR_PASSWDMISMATCH = 464,

    /// 465    ERR_YOUREBANNEDCREEP
    ///       ":You are banned from this server"
    /// 
    ///  - Returned after an attempt to connect and register
    ///    yourself with a server which has been setup to
    ///    explicitly deny connections to you.
    /// 
    ERR_YOUREBANNEDCREEP = 465,

    /// 466    ERR_YOUWILLBEBANNED
    /// 
    ///  - Sent by a server to a user to inform that access to the
    ///    server will soon be denied.
    /// 
    ERR_YOUWILLBEBANNED = 466,

    /// 467    ERR_KEYSET
    ///       "<channel> :Channel key already set"
    ERR_KEYSET = 467,

    /// 471    ERR_CHANNELISFULL
    ///       "<channel> :Cannot join channel (+l)"
    ERR_CHANNELISFULL = 471,

    /// 472    ERR_UNKNOWNMODE
    ///       "<char> :is unknown mode char to me for <channel>"
    ERR_UNKNOWNMODE = 472,

    /// 473    ERR_INVITEONLYCHAN
    ///       "<channel> :Cannot join channel (+i)"
    ERR_INVITEONLYCHAN = 473,

    /// 474    ERR_BANNEDFROMCHAN
    ///       "<channel> :Cannot join channel (+b)"
    ERR_BANNEDFROMCHAN = 474,

    /// 475    ERR_BADCHANNELKEY
    ///       "<channel> :Cannot join channel (+k)"
    ERR_BADCHANNELKEY = 475,

    /// 476    ERR_BADCHANMASK
    ///       "<channel> :Bad Channel Mask"
    ERR_BADCHANMASK = 476,

    /// 477    ERR_NOCHANMODES
    ///       "<channel> :Channel doesn't support modes"
    ERR_NOCHANMODES = 477,

    /// 478    ERR_BANLISTFULL
    ///       "<channel> <char> :Channel list is full"
    /// 
    ERR_BANLISTFULL = 478,

    /// 481    ERR_NOPRIVILEGES
    ///       ":Permission Denied- You're not an IRC operator"
    /// 
    ///  - Any command requiring operator privileges to operate
    ///    MUST return this error to indicate the attempt was
    ///    unsuccessful.
    /// 
    ERR_NOPRIVILEGES = 481,

    /// 482    ERR_CHANOPRIVSNEEDED
    ///       "<channel> :You're not channel operator"
    /// 
    ///  - Any command requiring 'chanop' privileges (such as
    ///    MODE messages) MUST return this error if the client
    ///    making the attempt is not a chanop on the specified
    ///    channel.
    /// 
    ERR_CHANOPRIVSNEEDED = 482,

    /// 483    ERR_CANTKILLSERVER
    ///       ":You can't kill a server!"
    /// 
    ///  - Any attempts to use the KILL command on a server
    ///    are to be refused and this error returned directly
    ///    to the client.
    /// 
    ERR_CANTKILLSERVER = 483,

    /// 484    ERR_RESTRICTED
    ///       ":Your connection is restricted!"
    /// 
    ///  - Sent by the server to a user upon connection to indicate
    ///    the restricted nature of the connection (user mode "+r").
    /// 
    ERR_RESTRICTED = 484,

    /// 485    ERR_UNIQOPPRIVSNEEDED
    ///       ":You're not the original channel operator"
    /// 
    ///  - Any MODE requiring "channel creator" privileges MUST
    ///    return this error if the client making the attempt is not
    ///    a chanop on the specified channel.
    /// 
    ERR_UNIQOPPRIVSNEEDED = 485,

    /// 491    ERR_NOOPERHOST
    ///       ":No O-lines for your host"
    /// 
    ///  - If a client sends an OPER message and the server has
    ///    not been configured to allow connections from the
    ///    client's host as an operator, this error MUST be
    ///    returned.
    /// 
    ERR_NOOPERHOST = 491,

    /// 501    ERR_UMODEUNKNOWNFLAG
    ///       ":Unknown MODE flag"
    /// 
    ///  - Returned by the server to indicate that a MODE
    ///    message was sent with a nickname parameter and that
    ///    the a mode flag sent was not recognized.
    /// 
    ERR_UMODEUNKNOWNFLAG = 501,

    /// 502    ERR_USERSDONTMATCH
    ///       ":Cannot change mode for other users"
    /// 
    ///  - Error sent to any user trying to view or change the
    ///    user mode for a user other than themselves.
    /// 
    ERR_USERSDONTMATCH = 502,

}


impl FromStr for Reply {
    type Err = IrscError;
    fn from_str(s: &str) -> Result<Reply> {
        use self::Reply::*;
        match s {
            "001" => Ok(RPL_WELCOME),
            "002" => Ok(RPL_YOURHOST),
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
            "502" => Ok(ERR_USERSDONTMATCH),
            _ => Err(IrscError::NotFound)
        }
     }
}
impl ToString for Reply {
    fn to_string(&self) -> String {
        use self::Reply::*;
        match *self {
            RPL_WELCOME => "001".to_owned(),
            RPL_YOURHOST => "002".to_owned(),
            RPL_CREATED => "003".to_owned(),
            RPL_MYINFO => "004".to_owned(),
            RPL_BOUNCE => "005".to_owned(),
            RPL_USERHOST => "302".to_owned(),
            RPL_ISON => "303".to_owned(),
            RPL_AWAY => "301".to_owned(),
            RPL_UNAWAY => "305".to_owned(),
            RPL_NOWAWAY => "306".to_owned(),
            RPL_WHOISUSER => "311".to_owned(),
            RPL_WHOISSERVER => "312".to_owned(),
            RPL_WHOISOPERATOR => "313".to_owned(),
            RPL_WHOISIDLE => "317".to_owned(),
            RPL_ENDOFWHOIS => "318".to_owned(),
            RPL_WHOISCHANNELS => "319".to_owned(),
            RPL_WHOWASUSER => "314".to_owned(),
            RPL_ENDOFWHOWAS => "369".to_owned(),
            RPL_LISTSTART => "321".to_owned(),
            RPL_LIST => "322".to_owned(),
            RPL_LISTEND => "323".to_owned(),
            RPL_UNIQOPIS => "325".to_owned(),
            RPL_CHANNELMODEIS => "324".to_owned(),
            RPL_NOTOPIC => "331".to_owned(),
            RPL_TOPIC => "332".to_owned(),
            RPL_INVITING => "341".to_owned(),
            RPL_SUMMONING => "342".to_owned(),
            RPL_INVITELIST => "346".to_owned(),
            RPL_ENDOFINVITELIST => "347".to_owned(),
            RPL_EXCEPTLIST => "348".to_owned(),
            RPL_ENDOFEXCEPTLIST => "349".to_owned(),
            RPL_VERSION => "351".to_owned(),
            RPL_WHOREPLY => "352".to_owned(),
            RPL_ENDOFWHO => "315".to_owned(),
            RPL_NAMREPLY => "353".to_owned(),
            RPL_ENDOFNAMES => "366".to_owned(),
            RPL_LINKS => "364".to_owned(),
            RPL_ENDOFLINKS => "365".to_owned(),
            RPL_BANLIST => "367".to_owned(),
            RPL_ENDOFBANLIST => "368".to_owned(),
            RPL_INFO => "371".to_owned(),
            RPL_ENDOFINFO => "374".to_owned(),
            RPL_MOTDSTART => "375".to_owned(),
            RPL_MOTD => "372".to_owned(),
            RPL_ENDOFMOTD => "376".to_owned(),
            RPL_YOUREOPER => "381".to_owned(),
            RPL_REHASHING => "382".to_owned(),
            RPL_YOURESERVICE => "383".to_owned(),
            RPL_TIME => "391".to_owned(),
            RPL_USERSSTART => "392".to_owned(),
            RPL_USERS => "393".to_owned(),
            RPL_ENDOFUSERS => "394".to_owned(),
            RPL_NOUSERS => "395".to_owned(),
            RPL_TRACELINK => "200".to_owned(),
            RPL_TRACECONNECTING => "201".to_owned(),
            RPL_TRACEHANDSHAKE => "202".to_owned(),
            RPL_TRACEUNKNOWN => "203".to_owned(),
            RPL_TRACEOPERATOR => "204".to_owned(),
            RPL_TRACEUSER => "205".to_owned(),
            RPL_TRACESERVER => "206".to_owned(),
            RPL_TRACESERVICE => "207".to_owned(),
            RPL_TRACENEWTYPE => "208".to_owned(),
            RPL_TRACECLASS => "209".to_owned(),
            RPL_TRACERECONNECT => "210".to_owned(),
            RPL_TRACELOG => "261".to_owned(),
            RPL_TRACEEND => "262".to_owned(),
            RPL_STATSLINKINFO => "211".to_owned(),
            RPL_STATSCOMMANDS => "212".to_owned(),
            RPL_ENDOFSTATS => "219".to_owned(),
            RPL_STATSUPTIME => "242".to_owned(),
            RPL_STATSOLINE => "243".to_owned(),
            RPL_UMODEIS => "221".to_owned(),
            RPL_SERVLIST => "234".to_owned(),
            RPL_SERVLISTEND => "235".to_owned(),
            RPL_LUSERCLIENT => "251".to_owned(),
            RPL_LUSEROP => "252".to_owned(),
            RPL_LUSERUNKNOWN => "253".to_owned(),
            RPL_LUSERCHANNELS => "254".to_owned(),
            RPL_LUSERME => "255".to_owned(),
            RPL_ADMINME => "256".to_owned(),
            RPL_ADMINLOC1 => "257".to_owned(),
            RPL_ADMINLOC2 => "258".to_owned(),
            RPL_ADMINEMAIL => "259".to_owned(),
            RPL_TRYAGAIN => "263".to_owned(),
            ERR_NOSUCHNICK => "401".to_owned(),
            ERR_NOSUCHSERVER => "402".to_owned(),
            ERR_NOSUCHCHANNEL => "403".to_owned(),
            ERR_CANNOTSENDTOCHAN => "404".to_owned(),
            ERR_TOOMANYCHANNELS => "405".to_owned(),
            ERR_WASNOSUCHNICK => "406".to_owned(),
            ERR_TOOMANYTARGETS => "407".to_owned(),
            ERR_NOSUCHSERVICE => "408".to_owned(),
            ERR_NOORIGIN => "409".to_owned(),
            ERR_NORECIPIENT => "411".to_owned(),
            ERR_NOTEXTTOSEND => "412".to_owned(),
            ERR_NOTOPLEVEL => "413".to_owned(),
            ERR_WILDTOPLEVEL => "414".to_owned(),
            ERR_BADMASK => "415".to_owned(),
            ERR_UNKNOWNCOMMAND => "421".to_owned(),
            ERR_NOMOTD => "422".to_owned(),
            ERR_NOADMININFO => "423".to_owned(),
            ERR_FILEERROR => "424".to_owned(),
            ERR_NONICKNAMEGIVEN => "431".to_owned(),
            ERR_ERRONEUSNICKNAME => "432".to_owned(),
            ERR_NICKNAMEINUSE => "433".to_owned(),
            ERR_NICKCOLLISION => "436".to_owned(),
            ERR_UNAVAILRESOURCE => "437".to_owned(),
            ERR_USERNOTINCHANNEL => "441".to_owned(),
            ERR_NOTONCHANNEL => "442".to_owned(),
            ERR_USERONCHANNEL => "443".to_owned(),
            ERR_NOLOGIN => "444".to_owned(),
            ERR_SUMMONDISABLED => "445".to_owned(),
            ERR_USERSDISABLED => "446".to_owned(),
            ERR_NOTREGISTERED => "451".to_owned(),
            ERR_NEEDMOREPARAMS => "461".to_owned(),
            ERR_ALREADYREGISTRED => "462".to_owned(),
            ERR_NOPERMFORHOST => "463".to_owned(),
            ERR_PASSWDMISMATCH => "464".to_owned(),
            ERR_YOUREBANNEDCREEP => "465".to_owned(),
            ERR_YOUWILLBEBANNED => "466".to_owned(),
            ERR_KEYSET => "467".to_owned(),
            ERR_CHANNELISFULL => "471".to_owned(),
            ERR_UNKNOWNMODE => "472".to_owned(),
            ERR_INVITEONLYCHAN => "473".to_owned(),
            ERR_BANNEDFROMCHAN => "474".to_owned(),
            ERR_BADCHANNELKEY => "475".to_owned(),
            ERR_BADCHANMASK => "476".to_owned(),
            ERR_NOCHANMODES => "477".to_owned(),
            ERR_BANLISTFULL => "478".to_owned(),
            ERR_NOPRIVILEGES => "481".to_owned(),
            ERR_CHANOPRIVSNEEDED => "482".to_owned(),
            ERR_CANTKILLSERVER => "483".to_owned(),
            ERR_RESTRICTED => "484".to_owned(),
            ERR_UNIQOPPRIVSNEEDED => "485".to_owned(),
            ERR_NOOPERHOST => "491".to_owned(),
            ERR_UMODEUNKNOWNFLAG => "501".to_owned(),
            ERR_USERSDONTMATCH => "502".to_owned(),
        }
     }
}
