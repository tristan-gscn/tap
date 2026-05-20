use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};

use crate::protocol::command::Command;
use crate::protocol::response::Response;
use crate::state::game::GameState;
use tracing::{debug, info};

/// Handles a single client connection.
/// Manages the full lifecycle of a session: reading commands, dispatching them, and writing responses.
pub async fn handle(socket: TcpStream, addr: String, state: Arc<RwLock<GameState>>) {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    let (tx, mut rx) = mpsc::unbounded_channel::<Response>();

    let writer_task = tokio::spawn(async move {
        while let Some(resp) = rx.recv().await {
            if writer.write_all(resp.to_line().as_bytes()).await.is_err() {
                break;
            }
        }
    });

    while let Ok(Some(line)) = lines.next_line().await {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        debug!("[{}] << {}", addr, line);

        let parsed: Result<Command, Response> = Command::parse(&line);

        let response: Response = match parsed {
            Ok(cmd) => super::dispatch::dispatch(cmd, &addr, &tx, Arc::clone(&state)).await,
            Err(e) => e,
        };

        if tx.send(response).is_err() {
            break;
        }
    }
    info!("Connection closed: {}", addr);
    super::handlers::session::disconnect(&addr, state).await;
    drop(tx);
    let _ = writer_task.await;
}