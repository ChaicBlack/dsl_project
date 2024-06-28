use dsl_project::{msg::Ping, Connection, Message};

use tokio;
use tokio::net::TcpStream;

use std::io;
use std::{collections::HashSet, net::SocketAddr};

pub struct Node {
    pub id: u32,
    pub name: String,
    pub addr: SocketAddr,
    pub neighbors: HashSet<SocketAddr>,
}

impl Node {
    pub fn new(id: u32, name: String, addr: &str, neighbors: Vec<&str>) -> Self {
        let neighbors = neighbors.into_iter().map(|s| s.parse().unwrap()).collect();

        Node {
            id,
            name,
            addr: addr.parse().unwrap(),
            neighbors,
        }
    }
}

#[tokio::main()]
async fn main() -> io::Result<()> {}
