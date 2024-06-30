use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::Config;

/// Server state shared across all connections.
///
/// `Db` contains a `HashMap` storing the key/value data.
///
/// A `Db` instance is a handle to shared state. Cloning `Db` is shallow and
/// only incurs an atomic ref count increment.
#[derive(Debug, Clone)]
pub(crate) struct Db {
    config: Config,
    /// Handle to log.
    ///
    /// The log is guarded by a mutex. This is a `std::sync::Mutex` and
    /// not a Tokio mutex. This is because there are no asynchronous operations
    /// being performed while holding the mutex. Additionally, the critical
    /// sections are very small.
    log: Arc<Mutex<HashMap<u64, String>>>,

    /// Handle the neighboring nodes' address.
    ///
    /// The key is the id of neighbor, the value is a std::net::SocketAddr of neighbor.
    neighbors: Arc<Mutex<HashMap<u64, SocketAddr>>>,
}

impl Db {
    pub(crate) fn new(config: &Config) -> Db {
        let log = Arc::new(Mutex::new(HashMap::new()));

        let neighbors = Arc::new(Mutex::new(HashMap::new()));

        Db {
            config: config.clone(),
            log,
            neighbors,
        }
    }

    /// Get the value associated with a key.
    pub(crate) fn get_log(&self, key: u64) -> Option<String> {
        let log = self.log.lock().unwrap();

        log.get(&key).map(|x| x.clone())
    }

    /// Set a value of a key.
    ///
    /// If a value already assosiated with the key, it's removed.
    pub(crate) fn set_log(&self, key: u64, value: &str) {
        let mut log = self.log.lock().unwrap();

        log.insert(key, value.to_string());
    }

    /// Get the value associated with a key.
    pub(crate) fn get_neighbor(&self, key: u64) -> Option<SocketAddr> {
        let neighbor = self.neighbors.lock().unwrap();

        neighbor.get(&key).map(|x| x.clone())
    }

    /// Set a value of a key.
    ///
    /// If a value already assosiated with the key, it's removed.
    pub(crate) fn set_neighbor(&self, key: u64, value: &SocketAddr) {
        let mut neighbor = self.neighbors.lock().unwrap();

        neighbor.insert(key, value.clone());
    }

    pub fn get_config(&self) -> Config {
        self.config.clone()
    }
}
