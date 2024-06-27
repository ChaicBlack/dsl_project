use crate::{Connection, Db, Frame, Parse};

use bytes::Bytes;
use tracing::{debug, instrument};

/// Get the value of key.
///
/// If the key does not exist the special value nil is returned. An error is
/// returned if the value stored at key is not a string, because GET only
/// handles string values.
pub struct Get {
    /// Name of the key to get.
    key: String,
}

impl Get {
    /// Create a new 'Get' message which fetches 'key'.
    pub fn new(key: impl ToString) -> Self {
        Get {
            key: key.to_string(),
        }
    }

    /// Get the key.
    pub fn key(&self) -> &str {
        &self.key
    }
}
