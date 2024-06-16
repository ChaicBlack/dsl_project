use bytes::{Buf, BytesMut};
use log::info;
use std::io::Cursor;
use std::net::SocketAddr;
use tokio::io::{self, AsyncWriteExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::frame::Frame;
use crate::node::Node;

pub struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // allocating a 4kb buffer
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // if there are a whole frame in the buffer, read it and return
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;

            // if not, read more data from socket into buffer
            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                self.cursor += n;
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {}

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;

                self.buffer.advance(len);

                Ok(Some(frame))
            }

            Err(Imcomplete) => Ok(None),

            Err(e) => Err(e.into()),
        }
    }
}

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
