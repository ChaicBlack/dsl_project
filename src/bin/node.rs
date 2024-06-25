use dsl_project::{frame, Connection};

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
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;

    let mut con = Connection::new(socket);

    let msg = frame::Frame::Simple("hello".to_string());

    con.write_frame(&msg).await?;

    Ok(())
}
