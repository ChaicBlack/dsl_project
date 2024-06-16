use log::info;
use std::net::SocketAddr;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::node::Node;

impl Node {
    //This need to be refactored after send_message been completed
    pub fn broadcast(&self, message: &str) {
        for neighbor in &self.neighbors {
            if let Err(e) = self.send_message(message, neighbor) {
                info!("Failed to send message to {}, {}", neighbor.to_string(), e);
                return;
            }
            info!("{} broadcast to neighbors.", self.addr.to_string());
        }
    }

    // Afte all I need to implement a dedicated task to handle IO using mpsc.
    pub async fn send_message(&self, message: &str, target: &SocketAddr) -> io::Result<()> {
        let socket = TcpStream::connect(target).await?;

        socket.write_frame(Frame).await?; // I need to implement my own Frame and connection just
                                          // like mini_redis in tutorial

        Ok(())
    }
}
