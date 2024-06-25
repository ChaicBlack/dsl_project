use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use bytes::Bytes;

/// Server state shared across all connections.
///
/// `Db` contains a `HashMap` storing the key/value data.
///
/// A `Db` instance is a handle to shared state. Cloning `Db` is shallow and
/// only incurs an atomic ref count increment.
#[derive(Debug, Clone)]
pub(crate) struct Db {
    /// Handle to log.
    ///
    /// The log is guarded by a mutex. This is a `std::sync::Mutex` and
    /// not a Tokio mutex. This is because there are no asynchronous operations
    /// being performed while holding the mutex. Additionally, the critical
    /// sections are very small.
    log: Arc<Mutex<HashMap<String, Bytes>>>,

    /// Handle the neighboring nodes' address.
    ///
    /// The key is the id of neighbor, the value is a std::net::SocketAddr of neighbor.
    neighbors: Arc<Mutex<HashMap<u64, SocketAddr>>>,
}

impl Db {
    pub(crate) fn new() -> Db {
        let log = Arc::new(Mutex::new(HashMap::new()));

        let neighbors = Arc::new(Mutex::new(HashMap::new()));

        Db { log, neighbors }
    }
}
