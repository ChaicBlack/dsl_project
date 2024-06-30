use crate::{config, Frame, Parse, ParseError};

use bytes::Bytes;

/// Message use for indicating the sender is alive.
///
/// Usually used with timeout event.
#[derive(Debug)]
pub struct HeartBeat {
    id: u64,
}

impl HeartBeat {
    pub(crate) fn new() -> HeartBeat {
        HeartBeat { id: config::ID }
    }

    /// Parse a `HeartBeat` instance from a received frame.
    ///
    /// The `HEARTBEAT` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `HeartBeat` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing two entries.
    ///
    /// ```text
    /// HEARTBEAT id
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<HeartBeat> {
        // The `HEARTBEAT` string has already been consumed. The next value is the
        // name of the id to get. If the next value is not a u64 or the
        // input is fully consumed, then an error is returned.
        let id = parse.next_int()?;

        Ok(HeartBeat { id })
    }

    /// Converts the message into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `HeartBeat` message to send to
    /// the server.
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("heartbeat".as_bytes()));
        frame.push_int(self.id);
        frame
    }
}
