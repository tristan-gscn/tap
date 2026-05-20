use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Returns a descriptive label for a given HP level.
fn status_label(hp: i32, max_hp: i32) -> &'static str {
    if hp <= 0 {
        "dead"
    } else if hp >= max_hp {
        "healthy"
    } else if hp * 4 <= max_hp {
        "critical"
    } else {
        "wounded"
    }
}

/// Handles the `STATUS` command.
/// Returns the player's current health status.
pub async fn status(addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    let player = &state.players[&name];
    Response::ok(
        "status",
        json!({
            "hp": player.hp,
            "max_hp": player.max_hp,
            "status": status_label(player.hp, player.max_hp),
        }),
    )
}
