mod create;
mod invite;
mod join;
mod leave;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::protocol::command::GroupAction;
use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Routes a GROUP action to the appropriate handler based on the subcommand.
pub async fn group(action: GroupAction, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    match action {
        GroupAction::Create => create::create(&mut state, &name),
        GroupAction::Invite { target } => invite::invite(&mut state, &name, target),
        GroupAction::Join { leader } => join::join(&mut state, &name, leader),
        GroupAction::Leave => leave::leave(&mut state, &name),
    }
}
