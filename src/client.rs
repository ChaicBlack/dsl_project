use crate::{msg::Ping, Connection, Frame};

use std::io;

use tokio::net::TcpStream;

pub async fn client() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;

    let mut con = Connection::new(socket);

    let msg = Ping::new(None);

    let frame = msg.into_frame();

    con.write_frame(&frame).await.unwrap();

    let res = con.read_frame().await.unwrap().unwrap();

    if let Frame::Simple(res) = res {
        println!("{}", res);
    }

    Ok(())
}
