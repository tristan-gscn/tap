#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bridge::run().await
}
