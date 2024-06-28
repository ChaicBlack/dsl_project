use dsl_project::server;

use tokio;

use std::io;

#[tokio::main()]
async fn main() -> io::Result<()> {
    server().await.unwrap();
    Ok(())
}
