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
}
