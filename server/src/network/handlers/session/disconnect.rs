use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::{GameState, GroupLeave};

/// Handles a player's disconnection.
/// Cleans up the player's state, handles group removal/disbanding, and notifies others in the room.
pub async fn disconnect(addr: &str, state: Arc<RwLock<GameState>>) {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return,
    };
    let room = state.players[&name].room.clone();

    match state.leave_group(&name) {
        GroupLeave::NotInGroup => {}
        GroupLeave::Left { remaining, .. } => {
            let msg = Response::ok(
                "event",
                json!({ "event": "group_leave", "name": name }),
            );
            for m in &remaining {
                state.send_to(m, msg.clone());
            }
        }
        GroupLeave::Disbanded { members, .. } => {
            let msg = Response::ok(
                "event",
                json!({ "event": "group_disband", "by": name }),
            );
            for m in &members {
                if m.as_str() != name {
                    state.send_to(m, msg.clone());
                }
            }
        }
    }

    state.players.remove(&name);

    state.broadcast_room(
        &room,
        None,
        Response::ok(
            "event",
            json!({ "event": "presence_leave", "name": name }),
        ),
    );

    info!(player = %name, "Player disconnected");
}
