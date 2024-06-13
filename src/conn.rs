use log::info;
use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
};

use crate::node::Node;

impl Node {
    pub fn broadcast(&self, message: &str) {
        for neighbor in &self.neighbors {
            if let Err(e) = self.send_message(message, neighbor) {
                info!("Failed to send message to {}, {}", neighbor.to_string(), e);
                return;
            }
            info!("{} broadcast to neighbors.", self.addr.to_string());
        }
    }

    pub fn send_message(&self, message: &str, target: &SocketAddr) -> io::Result<()> {
        match TcpStream::connect(target) {
            Ok(mut stream) => {
                stream.write_all(message.as_bytes())?;
                info!(
                    "message {} from {} sent to {}.",
                    message,
                    self.addr.to_string(),
                    target.to_string()
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
