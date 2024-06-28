use dsl_project::client;

use tokio;

#[tokio::main()]
async fn main() -> std::io::Result<()> {
    client().await.unwrap();
    Ok(())
}
