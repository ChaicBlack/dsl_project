use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    id: u64,
    addr: SocketAddr,
}

impl Config {
    pub fn new(id: u64, addr: SocketAddr) -> Config {
        Config { id, addr }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
