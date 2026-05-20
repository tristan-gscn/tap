use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the `QUIT` command from a client.
/// Removes the player from the game state. Note: the actual socket closure is handled in the connection loop.
pub async fn quit(addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(201, "You are not connected"),
    };

    state.players.remove(&name);
    Response::ok("quit", json!({ "message": "bye" }))
}
