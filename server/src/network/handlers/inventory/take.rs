use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::config;
use crate::network::handlers::quest::notify_progress;
use crate::network::handlers::resolve_item;
use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the TAKE command, allowing a player to pick up an item from the room.
pub async fn take(query: String, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room = state.players[&name].room.clone();

    let item_id = match resolve_item(&query) {
        Some(id) => id,
        None => return Response::error(404, "ITEM_NOT_FOUND"),
    };

    let present = state.world.items_in(&room).iter().any(|i| i == &item_id);
    if !present {
        return Response::error(404, "ITEM_NOT_FOUND");
    }

    let obtainable = config::get()
        .world
        .items
        .get(&item_id)
        .map(|i| i.obtainable)
        .unwrap_or(false);
    if !obtainable {
        return Response::error(404, "ITEM_NOT_FOUND");
    }

    state.world.remove_item(&room, &item_id);
    let touched = state
        .players
        .get_mut(&name)
        .map(|p| {
            p.inventory.push(item_id.clone());
            p.advance_quests("collect", &item_id, 1)
        })
        .unwrap_or_default();

    state.broadcast_room(
        &room,
        None,
        Response::ok(
            "event",
            json!({ "event": "item_taken", "by": name, "item": item_id }),
        ),
    );

    notify_progress(&state, &name, &touched);

    info!(player = %name, item = %item_id, "Item taken");
    Response::ok("take", json!({ "taken": item_id }))
}
