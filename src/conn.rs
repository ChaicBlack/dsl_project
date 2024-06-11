use std::{
    io::{self, Write},
    net::TcpStream,
};

use log::info;

use crate::node::Node;

impl Node {
    pub fn send_message(&self, message: &str, address: &str) -> io::Result<()> {
        let mut stream = TcpStream::connect(address)?;
        stream.write_all(message.as_bytes())?;
        info!("Node {} - {} sent message: {}", self.id, self.name, message);
        Ok(())
    }
}
