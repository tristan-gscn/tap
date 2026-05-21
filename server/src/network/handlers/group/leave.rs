use serde_json::json;
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::{GameState, GroupLeave};

/// Handles the GROUP LEAVE subcommand, removing the player from their group.
pub fn leave(state: &mut GameState, name: &str) -> Response {
    match state.leave_group(name) {
        GroupLeave::NotInGroup => Response::error(401, "NOT_IN_GROUP"),

        GroupLeave::Left { gid, remaining } => {
            let msg = Response::ok("event", json!({ "event": "group_leave", "name": name }));
            for m in &remaining {
                state.send_to(m, msg.clone());
            }
            info!(player = %name, group = gid, "Group leave");
            Response::ok("group", json!({ "action": "leave" }))
        }

        GroupLeave::Disbanded { gid, members } => {
            let msg = Response::ok("event", json!({ "event": "group_disband", "by": name }));
            for m in &members {
                if m.as_str() != name {
                    state.send_to(m, msg.clone());
                }
            }
            info!(player = %name, group = gid, "Group disbanded");
            Response::ok("group", json!({ "action": "leave", "disbanded": true }))
        }
    }
}
