use crate::Frame;

use bytes::Bytes;
use std::{fmt, str, vec};

/// Utility for parsing a command
///
/// Commands are represented as array frames. Each entry in the frame is a
/// "token". A `Parse` is initialized with the array frame and provides a
/// cursor-like API. Each command struct includes a `parse_frame` method that
/// uses a `Parse` to extract its fields.
#[derive(Debug)]
pub(crate) struct Parse {
    /// Array frame iterator.
    parts: vec::IntoIter<Frame>,
}

/// Error encountered while parsing a frame.
///
/// Only 'EndOfStream' errors are handled at runtime. All other errors result in
/// the connection being terminated.
pub(crate) enum ParseError {
    /// Attempting to extract a value failed due to the frame being fully
    /// consumed.
    EndOfStream,

    /// All other errors.
    Other(crate::Error),
}

impl Parse {
    /// Create a new 'Parse' to parse the content of 'frame'.
    ///
    /// Returns 'Err' if 'frame' is not an  array frame.
    pub(crate) fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(format!("protocol error; expected array, got {:?}", frame).into()),
        };

        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    /// Return the next entry. Array frames are arrays of frames, so the next
    /// entry is a frame.
    fn next(&mut self) -> Result<Frame, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> ParseError {
        ParseError::Other(src.into())
    }
}
