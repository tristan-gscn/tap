use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::network::handlers::resolve_item;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub async fn drop_item(query: String, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room = state.players[&name].room.clone();

    let item_id = match resolve_item(&query) {
        Some(id) => id,
        None => return Response::error(404, "ITEM_NOT_IN_INVENTORY"),
    };

    let had = state
        .players
        .get_mut(&name)
        .map(|p| p.take_from_inventory(&item_id))
        .unwrap_or(false);
    if !had {
        return Response::error(404, "ITEM_NOT_IN_INVENTORY");
    }

    state.world.add_item(&room, item_id.clone());

    state.broadcast_room(
        &room,
        None,
        Response::ok(
            "event",
            json!({ "event": "item_dropped", "by": name, "item": item_id }),
        ),
    );

    info!(player = %name, item = %item_id, "Item dropped");
    Response::ok("drop", json!({ "dropped": item_id }))
}
