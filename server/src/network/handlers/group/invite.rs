use serde_json::json;
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the GROUP INVITE subcommand, inviting a player to the player's group.
pub fn invite(state: &mut GameState, name: &str, target: String) -> Response {
    let gid = match state.players[name].group {
        Some(gid) => gid,
        None => return Response::error(401, "NOT_IN_GROUP"),
    };
    if target.as_str() == name {
        return Response::error(400, "Cannot invite yourself");
    }
    if !state.players.contains_key(&target) {
        return Response::error(404, "Player not found");
    }
    if state.players[&target].group.is_some() {
        return Response::error(402, "ALREADY_IN_GROUP");
    }

    state.invite_to_group(gid, &target);
    let leader = state.groups[&gid].leader.clone();

    state.send_to(
        &target,
        Response::ok(
            "event",
            json!({
                "event": "group_invite",
                "leader": leader,
                "group": gid,
                "from": name,
            }),
        ),
    );

    info!(player = %name, target = %target, group = gid, "Group invite");
    Response::ok("group", json!({ "action": "invite", "invited": target }))
}
