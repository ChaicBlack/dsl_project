use crate::{Db, Frame, Parse, ParseError};

use bytes::Bytes;

/// Message use for indicating the sender is alive.
///
/// Usually used with timeout event.
#[derive(Debug)]
pub struct HeartBeat {
    id: u64,
}

impl HeartBeat {
    pub fn new(db: &Db) -> HeartBeat {
        HeartBeat {
            id: db.get_config().get_id(),
        }
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<HeartBeat> {
        let id = parse.next_int()?;

        Ok(HeartBeat { id })
    }

    pub(crate) async fn apply(self, db: &Db) -> crate::Result<()> {
        // need to be added.
        Ok(())
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("heartbeat".as_bytes()));
        frame.push_int(self.id);
        frame
    }
}
