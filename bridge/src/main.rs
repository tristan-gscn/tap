/// Main entry point for the bridge binary.
/// Initializes and runs the bridge server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bridge::run().await
}
