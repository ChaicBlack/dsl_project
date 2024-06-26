mod get;

mod heartbeat;

mod set;

mod ping;

use crate::{Connection, Db, Frame, Parse, ParseError};

#[derive(Debug)]
pub enum Message {
    Get(get),
    HeartBeat(HeartBeat),
    Set(Set),
    Ping(Ping),
}
