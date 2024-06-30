use crate::{config, Connection, Db, Message};

use tokio;
use tokio::net::TcpListener;

use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

pub async fn server() -> io::Result<()> {
    let listener = TcpListener::bind(config::ADDR).await?;

    println!("Server running on {}", &config::ADDR);

    let db = Db::new();

    loop {
        let (socket, _) = listener.accept().await?;

        let db = db.clone();

        tokio::task::spawn(async move {
            let mut con = Connection::new(socket);

            let frame = con.read_frame().await.unwrap().unwrap();

            let msg = Message::from_frame(frame).unwrap();

            println!("{}", msg.get_name());

            msg.apply(&db, &mut con).await.unwrap();
        });
    }
}
