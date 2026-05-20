use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the INVENTORY command, returning a list of items the player is carrying.
pub async fn inventory(addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    let items = state.players[&name].inventory.clone();
    Response::ok("inventory", json!({ "items": items }))
}
