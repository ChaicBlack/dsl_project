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
    // id needs to be allocated serially or ask server(Does it means I need to implement server
    // node?)
    pub fn new(id: u32, name: String, log_file_path: &str) -> Self {
        let log_file = File::create(log_file_path).unwrap();
        WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

        Self {
            id,
            name,
            neighbors: Vec::new(),
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
