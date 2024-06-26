mod get;
pub use get::Get;

mod heartbeat;
pub use heartbeat::HeartBeat;

mod set;
pub use set::Set;

mod ping;
pub use ping::Ping;

use crate::{Connection, Db, Frame, Parse, ParseError};

/// Enumeration of supported message types.
///
/// Methods called on 'Message' are delegated to the Message implementation.
#[derive(Debug)]
pub enum Message {
    Get(get),
    HeartBeat(HeartBeat),
    Set(Set),
    Ping(Ping),
    Unknown(Unknown),
}

impl Message {
    /// Parse a message from a received frame.
    ///
    /// The 'Frame' must represent a message supported by 'Message' and be the array
    /// variant.
    ///
    /// # Returns
    ///
    /// On success, the message value is returned, otherwise, 'Err' is returned.
    pub fn from_frame(frame: Frame) -> crate::Result<Message> {
        let mut parse = Parse::new(frame)?;

        // All message begin with the message name as a string. The name is read and
        // converted to lower cases in order to do case sensitive matching.
        let message_name = parse.next_string()?.to_lowercase();

        // Matching the message name, delegating the rest of the parsing to the specific
        // message.
        let message = match &message_name[..] {
            "get" => Message::Get(Get::parse_frames(&mut parse)?),
            "heartbeat" => Message::HeartBeat(HeartBeat::parse_frames(&mut parse)?),
            "set" => Message::Set(Set::parse_frames(&mut parse)?),
            "ping" => Message::Ping(Ping::parse_frames(&mut parse)?),
            _ => {
                // The message is not recognized and an Unknown message is
                // returned.
                //
                // `return` is called here to skip the `finish()` call below. As
                // the message is not recognized, there is most likely
                // unconsumed fields remaining in the `Parse` instance.
                return Ok(Message::Unknown(Unknown::new(message_name)));
            }
        };

        // Check if there is any remaining unconsumed fields in the `Parse`
        // value. If fields remain, this indicates an unexpected frame format
        // and an error is returned.
        parse.finish()?;

        // The message has been successfully parsed
        Ok(message)
    }

    /// Apply the operation counter to specific message type.
    ///
    /// This is called by the nodes that receives a message.
    pub(crate) async fn apply(self, db: Db, dst: &mut Connection) -> crate::Result<()> {
        use Message::*;

        match self {
            Get(msg) => msg.apply(db, dst).await,
            HeartBeat(msg) => msg.apply(dst).await,
            Set(msg) => msg.apply(db, dst).await,
            Ping(msg) => msg(dst).await,
            Unknown(msg) => msg.apply(dst).await,
        }
    }

    /// Returns the message name.
    pub(crate) fn get_name(&self) -> &str {
        match self {
            Message::Get(_) => "get",
            Message::HeartBeat(_) => "heartbeat",
            Message::Set(_) => "set",
            Message::Ping(_) => "ping",
            Message::Unknown(msg) => msg.get_name(),
        }
    }
}
