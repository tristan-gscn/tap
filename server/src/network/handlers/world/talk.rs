use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::config;
use crate::network::handlers::resolve_npc;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub async fn talk(query: String, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room = state.players[&name].room.clone();

    let npc_type = match resolve_npc(&query) {
        Some(t) => t,
        None => return Response::error(404, "NPC_NOT_FOUND"),
    };

    let present = state
        .world
        .npcs_in(&room)
        .iter()
        .any(|n| n.npc_type == npc_type);
    if !present {
        return Response::error(404, "NPC_NOT_FOUND");
    }

    let npc = match config::get().world.npcs.get(&npc_type) {
        Some(n) => n,
        None => return Response::error(404, "NPC_NOT_FOUND"),
    };

    info!(player = %name, npc = %npc_type, "Talk");
    Response::ok(
        "talk",
        json!({
            "npc": npc_type,
            "name": npc.name,
            "description": npc.description,
            "dialogue": npc.dialogue,
        }),
    )
}
