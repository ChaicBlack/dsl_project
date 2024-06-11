use log::{info, LevelFilter};
use simplelog::{Config, WriteLogger};
use std::fs::File;

pub struct Node {
    pub id: u32,
    pub name: String,
}

impl Node {
    // id needs to be allocated serially or ask server(Does it means I need to implement server
    // node?)
    pub fn new(id: u32, name: String, log_file_path: &str) -> Self {
        let log_file = File::create(log_file_path).unwrap();
        WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

        Self { id, name }
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
