use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::config;
use crate::config::Direction;
use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the MOVE command, allowing a player to move to an adjacent room.
pub async fn move_player(
    direction: Direction,
    addr: &str,
    state: Arc<RwLock<GameState>>,
) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let from = state.players[&name].room.clone();

    let cfg = config::get();
    let dest = match cfg
        .world
        .locations
        .get(&from)
        .and_then(|loc| loc.exits.get(&direction))
    {
        Some(d) => d.clone(),
        None => return Response::error(404, "NO_EXIT"),
    };

    if !cfg.world.locations.contains_key(&dest) {
        return Response::error(500, "Destination room not found in world data");
    }

    if let Some(p) = state.players.get_mut(&name) {
        p.room = dest.clone();
    }

    state.broadcast_room(
        &from,
        None,
        Response::ok(
            "event",
            json!({
                "event": "presence_leave",
                "name": name,
                "direction": direction.as_str(),
                "to": dest,
            }),
        ),
    );
    state.broadcast_room(
        &dest,
        Some(&name),
        Response::ok(
            "event",
            json!({ "event": "presence_enter", "name": name, "from": from }),
        ),
    );

    info!(player = %name, from = %from, to = %dest, "Player moved");
    Response::ok(
        "move",
        json!({ "from": from, "to": dest, "direction": direction.as_str() }),
    )
}
