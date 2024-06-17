use log::info;
use std::io::Cursor;
use std::net::SocketAddr;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::frame::{self, Frame};
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

    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
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

    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Array(val) => {
                self.stream.write_u8(b'*').await?;

                self.write_decimal(val.len() as u64).await?;

                for entry in &**val {
                    self.write_value(entry).await?;
                }
            }

            _ => self.write_value(frame).await?;
        }

        // The calls above are to the buffered stream and writes.
        self.stream.flush().await
    }

    async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'*').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }

            Frame::Array(_val) => unreachable!(),
        }

        Ok(())
    }

    // write a decimal frame to the stream
    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        use std::io::Write;

        // convert the value to a string
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val);

        let pos = buf.position() as usize;
        // get_ref() return the reference to inner value
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        use frame::Error::Imcomplete;

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
