use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::network::handlers::resolve_item;
use crate::protocol::response::Response;
use crate::state::game::GameState;
use crate::state::player::EquipSlot;

fn slot_label(slot: EquipSlot) -> &'static str {
    match slot {
        EquipSlot::Right => "right",
        EquipSlot::Left => "left",
    }
}

pub async fn equip(
    slot: EquipSlot,
    query: String,
    addr: &str,
    state: Arc<RwLock<GameState>>,
) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    let item_id = match resolve_item(&query) {
        Some(id) => id,
        None => return Response::error(404, "ITEM_NOT_IN_INVENTORY"),
    };

    let has_item = state
        .players
        .get(&name)
        .map(|p| p.inventory.iter().any(|i| i == &item_id))
        .unwrap_or(false);
    if !has_item {
        return Response::error(404, "ITEM_NOT_IN_INVENTORY");
    }

    if let Some(p) = state.players.get_mut(&name) {
        p.equip(slot, item_id.clone());
    }

    Response::ok(
        "equip",
        json!({ "slot": slot_label(slot), "item": item_id }),
    )
}

pub async fn unequip(slot: EquipSlot, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    if let Some(p) = state.players.get_mut(&name) {
        p.unequip(slot);
    }

    Response::ok("equip", json!({ "slot": slot_label(slot), "item": null }))
}
