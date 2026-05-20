use std::sync::Arc;

use serde_json::json;
use tokio::sync::{mpsc, RwLock};
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::GameState;
use crate::state::player::Player;

/// Handles a player's initial connection and registration with a name.
/// Broadcasts a presence event to other players in the same room.
pub async fn connect(
    name: String,
    class: Option<String>,
    addr: &str,
    tx: &mpsc::UnboundedSender<Response>,
    state: Arc<RwLock<GameState>>,
) -> Response {
    let mut state = state.write().await;

    if state.players.contains_key(&name) {
        return Response::error(201, "NAME_IN_USE");
    }

    let player = Player::new(name.clone(), addr.to_string(), class.clone(), tx.clone());
    let room = player.room.clone();
    state.players.insert(name.clone(), player);

    state.broadcast_room(
        &room,
        None,
        Response::ok(
            "event",
            json!({
                "event": "presence_enter",
                "name": name,
                "class": class,
            }),
        ),
    );

    info!(player = %name, class = ?class, "Player joined");
    Response::ok(
        "connect",
        json!({ "name": name, "class": class }),
    )
}
