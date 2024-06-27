PinBoard:
    1. The 'Frame' is actually RESP(Redis serialization protocol), a fast and easy to implement serialization protocol.
    2. The message types need to be extended later, carrying sender's infomation.
    3. The 'apply' method of each message type need to be extended when operations need.
    4. Need to implement snapshot, using serde. And maybe referencing to RDB and AOF of redis can help.
    5. Need to use snapshot as crash-recovery methods.

28/Jun/2024 night(2)
Completed:
    The Set message.
    The Unknown message.

28/Jun/2024 night
Completed:
    The Ping message, looks like I don't have to implement heartbeat because it is identical with Ping.

28/Jun/2024 evening
Several things to say:
    1. The 'Frame' is actually RESP(Redis serialization protocol), a fast and easy to implement serialization protocol.
    2. The message type need to be extended later, carrying sender's infomation.
    3. The 'apply' method of each message type need to be extended when operations need.
    4. Need to implement snapshot, using serde. And maybe referencing to RDB and AOF of redis can help.
    5. Need to use snapshot as crash-recovery methods.

27/Jun/2024 afternoon
Completed:
    Implementation of Get message, notice that this is different from Get command in redis because this key is u64 and value is String while in redis is String and bytes.
To be completed:
    other message type.
    server and client.

26/Jun/2024 evening
Completed:
    The skeleton of msg.rs, there could be message type in the future.
    By the way, I need to think about the operation by the node self, for example propose in Paxos.
To be Completed:
    I need to read the raft, viewstamp and pbft paper, to get the operation they use. There must be more in the Db and Message.
    And the specific message operation need to be implemented.

25/Jun/2024 night
Completed:
    Completed the db.rs, think a lot about my project:
    I can use frameworks like serde, tonic, etc. But those frameworks are too big
    and I don't need that many features because I just need a little of them.
    At first I failed to build a demo on my conn and frame mod. Then I began to use tokio_util
    to complete this. Later I found that there was a mistake on my implemtation of redis protocol.
    After fixed that, things worked. I can use those frames to build various command and messages.
To be completed:
    heartbeat, broadcast and snapshoot.
    and I need to implement backup when needed.

25/Jun/2024 afternoon
Completed:
    Do something on the Db. The Db will be responsible for use for handle data
    which maybe visited repeatly. Such as log and neighbor nodes.
To be completed:
    I need to implement a short version of ping-pong.

24/Jun/2024 night
Completed:
    added some comments for conn.rs and frame.rs.
   complete the frame.rs.
   start a db.rs.
To be completed:
   I need to use a db intance for the library because asynchronous operations could lead to data race.

23/Jun/2024 night
Completed:
    next_bytes, next_string, next_int, parse.rs.
To be completed:
  Feaguring out my own cmd list.
  After some thinking, I need parser libraries to translate dsl to rust
  code.
  I need to implement neighbor and broadcast first.

23/Jun/2024 morning
Completed:
    struct Parse, enum ParseError, From<String> for ParseError, impl new and next for Parse.
    Feagured out what the meaning of cmd and parse.
To be completed:
    The remainning of parse.rs, and feaguring out my own cmd list.

20/Jun/2024 night
Completed:
    get_decimal, get_line, from, try_into, impl others
To be completed:
    implementing ping pong between 2 nodes.

18/Jun/2024 1am
Completed:
    parse,
To be completed:
    to_vec, into, advance, get_line, get_decimal, try_into

17/Jun/2024 afternoon
Completed:
    Feagured out what cursor for.
    write_decimal, skip, peek_u8, get_u8
To be completed:
    parse, advance, into, get_line, get_decimal, try_into.

16/Jun/2024 night
Completed:
    Feagured out what frame is.
    write_frame, write_value, check.
To be completed:
    write_decimal, parse, advance, into, get_u8, get_line, get_decimal, skip, peek_u8, try_into.

16/Jun/2024
1. Frame, Frame::check, Frame::parse.
2. Result.
3. buffer::advance.
4. Any way, check the source code of mini-redis and implement the same way.

15/Jun/2024
1. This could be a library crate for use later, but now I need to implement the Paxos example.
2. Value transferred by network should be seriallized. But now just use u32 to keep simple.
3. Connection and messages sending need to be asynchronously implememted by Tokio.
4. I need several other instances using this code to be proposer, acceptor and learner. I am not sure if I can place tham in a bin folder like in tokio tutorial while keeping use the lib code.
