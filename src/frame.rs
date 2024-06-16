pub enum Frame {
    Heartbeat(u32), // id number
    Prepare {
        id: u32,
        proposal_number: u32,
    },
    Promise {
        id: u32,
        proposal_number: u32,
    },
    Request {
        id: u32,
        proposal_number: u32,
        value: u32, // to be changed to String or Bytes or even serialized
    },
    Accepted {
        id: u32,
        proposal_number: u32,
        value: u32,
    },
}
