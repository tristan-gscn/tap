use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::protocol::response::Response;
use crate::state::game::GameState;

pub async fn inventory(addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    let cfg = crate::config::get();
    let player = &state.players[&name];
    let items = player.inventory.clone();
    let items_detail: Vec<_> = items
        .iter()
        .map(|id| {
            let display = cfg
                .world
                .items
                .get(id)
                .map(|i| i.name.as_str())
                .unwrap_or(id.as_str());
            json!({ "id": id, "name": display })
        })
        .collect();
    Response::ok(
        "inventory",
        json!({
            "items": items,
            "items_detail": items_detail,
            "equipped": {
                "right": player.equipped_right,
                "left": player.equipped_left,
            }
        }),
    )
}
