use serde_json::json;
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::GameState;

pub fn join(state: &mut GameState, name: &str, leader: String) -> Response {
    if state.players[name].group.is_some() {
        return Response::error(402, "ALREADY_IN_GROUP");
    }
    let gid = match state.group_by_leader(&leader) {
        Some(gid) => gid,
        None => return Response::error(404, "No such group"),
    };
    if !state.is_invited(gid, name) {
        return Response::error(403, "Not invited to this group");
    }

    state.join_group(gid, name);

    state.broadcast_group(
        gid,
        None,
        Response::ok(
            "event",
            json!({ "event": "group_join", "name": name }),
        ),
    );

    info!(player = %name, group = gid, "Group join");
    Response::ok("group", json!({ "action": "join", "group": gid }))
}
