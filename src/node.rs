use log::{info, LevelFilter};
use simplelog::{Config, WriteLogger};
use std::{collections::HashSet, fs::File, net::SocketAddr};

pub struct Node {
    pub id: u32,
    pub name: String,
    pub addr: SocketAddr,
    pub neighbors: HashSet<SocketAddr>,
}

impl Node {
    pub fn new(
        id: u32,
        name: String,
        log_file_path: &str,
        addr: &str,
        neighbors: Vec<&str>,
    ) -> Self {
        let log_file = File::create(log_file_path).unwrap();
        WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

        let neighbors = neighbors.into_iter().map(|s| s.parse().unwrap()).collect();

        Node {
            id,
            name,
            addr: addr.parse().unwrap(),
            neighbors,
        }
    }

    pub fn do_something(&self) {
        info!("Node {} - {} is doing something.", self.id, self.name);
    }

    pub fn start(&self) {
        info!("Node {} - {} is starting.", self.id, self.name);
    }

    pub fn stop(&self) {
        info!("Node {} - {} is stopping.", self.id, self.name);
    }
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
