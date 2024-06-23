15/Jun/2024
1. This could be a library crate for use later, but now I need to implement the Paxos example.
2. Value transferred by network should be seriallized. But now just use u32 to keep simple.
3. Connection and messages sending need to be asynchronously implememted by Tokio.
4. I need several other instances using this code to be proposer, acceptor and learner. I am not sure if I can place tham in a bin folder like in tokio tutorial while keeping use the lib code.

16/Jun/2024
1. Frame, Frame::check, Frame::parse.
2. Result.
3. buffer::advance.
4. Any way, check the source code of mini-redis and implement the same way.

16/Jun/2024 night
Completed:
    Feagured out what frame is.
    write_frame, write_value, check.
To be completed:
    write_decimal, parse, advance, into, get_u8, get_line, get_decimal, skip, peek_u8, try_into.

17/Jun/2024 afternoon
Completed:
    Feagured out what cursor for.
    write_decimal, skip, peek_u8, get_u8
To be completed:
    parse, advance, into, get_line, get_decimal, try_into.

18/Jun/2024 1am
Completed:
    parse,
To be completed:
    to_vec, into, advance, get_line, get_decimal, try_into

20/Jun/2024 night
Completed:
    get_decimal, get_line, from, try_into, impl others
To be completed:
    implementing ping pong between 2 nodes.

23/Jun/2024 morning
Completed:
    struct Parse, enum ParseError, From<String> for ParseError, impl new and next for Parse.
    Feagured out what the meaning of cmd and parse.
To be completed:
    The remainning of parse.rs, and feaguring out my own cmd list.
