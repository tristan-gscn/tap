use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the `WHO` command.
/// Returns a list of all currently connected players.
pub async fn who(state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;
    let names: Vec<&String> = state.players.keys().collect();
    Response::ok(
        "who",
        json!({ "players": names, "count": names.len() }),
    )
}
