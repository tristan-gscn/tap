use std::env;

use tokio::net::TcpListener;

mod server;

/// Starts the bridge server.
///
/// Reads `TAP_BRIDGE_ADDR` and `TAP_SERVER_ADDR` environment variables
/// to determine the listening and destination addresses.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ws_addr = env::var("TAP_BRIDGE_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
    let tap_addr = env::var("TAP_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:4000".to_string());

    let listener = TcpListener::bind(&ws_addr).await?;
    println!("TAP bridge listening on ws://{}", ws_addr);
    println!("TAP server target tcp://{}", tap_addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let tap_addr = tap_addr.clone();
        tokio::spawn(async move {
            if let Err(err) = server::handle_client(stream, tap_addr).await {
                eprintln!("bridge client error: {}", err);
            }
        });
    }
}
