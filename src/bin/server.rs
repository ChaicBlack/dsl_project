use dsl_project::{frame, Connection};

use tokio;
use tokio::net::TcpListener;

use std::io;

#[tokio::main()]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    let (socket, _) = listener.accept().await?;

    let mut con = Connection::new(socket);

    let msg = con.read_frame().await.unwrap();

    println!("{:?}", msg);

    Ok(())
}
