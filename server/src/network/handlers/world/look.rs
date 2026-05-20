use std::sync::Arc;

use serde_json::{json, Map, Value};
use tokio::sync::RwLock;

use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

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

    let players_detail: Vec<Value> = state
        .players
        .values()
        .filter(|p| p.room == room_id)
        .map(|p| {
            json!({
                "name": p.name,
                "class": p.class,
                "hp": p.hp,
                "max_hp": p.max_hp,
            })
        })
        .collect();

    let npcs: Vec<Value> = state
        .world
        .npcs_in(&room_id)
        .iter()
        .map(|n| {
            let cfg_npc = cfg.world.npcs.get(&n.npc_type);
            let display_name = cfg_npc.map(|c| c.name.as_str()).unwrap_or(n.npc_type.as_str());
            let model = cfg_npc.and_then(|c| c.model.clone());
            json!({
                "id": n.id,
                "type": n.npc_type,
                "name": display_name,
                "model": model,
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
                "kind": loc.kind_or_default(),
                "name": loc.name,
                "description": loc.description,
                "exits": exits,
            },
            "players": players,
            "players_detail": players_detail,
            "items": state.world.items_in(&room_id),
            "items_detail": state.world.items_in(&room_id).iter().map(|id| {
                let name = cfg.world.items.get(id).map(|i| i.name.as_str()).unwrap_or(id.as_str());
                json!({ "id": id, "name": name })
            }).collect::<Vec<_>>(),
            "npcs": npcs,
        }),
    )
}
