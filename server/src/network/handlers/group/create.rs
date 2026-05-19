use serde_json::json;
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::GameState;

pub fn create(state: &mut GameState, name: &str) -> Response {
    if state.players[name].group.is_some() {
        return Response::error(402, "ALREADY_IN_GROUP");
    }
    let gid = state.create_group(name);
    info!(player = %name, group = gid, "Group created");
    Response::ok("group", json!({ "action": "create", "group": gid }))
}
