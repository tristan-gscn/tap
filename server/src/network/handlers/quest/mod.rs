mod accept;
mod complete;
mod list;
mod progress;
mod status;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::config;
use crate::protocol::command::QuestAction;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub use progress::notify_progress;

pub async fn quest(action: QuestAction, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    match action {
        QuestAction::List => list::list(&state, &name),
        QuestAction::Status => status::status(&state, &name),
        QuestAction::Accept { id } => accept::accept(&mut state, &name, id),
        QuestAction::Complete { id } => complete::complete(&mut state, &name, id),
    }
}

pub fn resolve_quest(query: &str) -> Option<String> {
    let q = query.trim();
    config::get()
        .world
        .quests
        .keys()
        .find(|id| id.eq_ignore_ascii_case(q))
        .cloned()
}
