use std::sync::Arc;

use serde_json::{json, Map, Value};
use tokio::sync::RwLock;

use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the LOOK command, returning information about the current room.
pub async fn look(addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room_id = state.players[&name].room.clone();

    let cfg = config::get();
    let loc = match cfg.world.locations.get(&room_id) {
        Some(l) => l,
        None => return Response::error(500, "Room not found in world data"),
    };

    let mut exits = Map::new();
    for (dir, dest) in &loc.exits {
        exits.insert(dir.as_str().to_string(), Value::String(dest.clone()));
    }

    let players: Vec<&String> = state
        .players
        .values()
        .filter(|p| p.room == room_id)
        .map(|p| &p.name)
        .collect();

    let npcs: Vec<Value> = state
        .world
        .npcs_in(&room_id)
        .iter()
        .map(|n| {
            json!({
                "id": n.id,
                "type": n.npc_type,
                "hp": n.hp,
                "max_hp": n.max_hp,
            })
        })
        .collect();

    Response::ok(
        "look",
        json!({
            "room": {
                "id": room_id,
                "name": loc.name,
                "description": loc.description,
                "exits": exits,
            },
            "players": players,
            "items": state.world.items_in(&room_id),
            "npcs": npcs,
        }),
    )
}
