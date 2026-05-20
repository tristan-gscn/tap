use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};

use crate::network::handlers::{chat, combat, group, inventory, quest, session, world};
use crate::protocol::command::Command;
use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Routes a parsed command to the appropriate logic handler.
/// Returns a `Response` that will be sent back to the client.
pub async fn dispatch(
    cmd: Command,
    addr: &str,
    tx: &mpsc::UnboundedSender<Response>,
    state: Arc<RwLock<GameState>>,
) -> Response {
    match cmd {
        Command::Connect { name } => session::connect(name, addr, tx, state).await,
        Command::Who => session::who(state).await,
        Command::Status => session::status(addr, state).await,
        Command::Look => world::look(addr, state).await,
        Command::Chat { scope, text } => chat::chat(scope, text, addr, state).await,
        Command::Take { item } => inventory::take(item, addr, state).await,
        Command::Drop { item } => inventory::drop_item(item, addr, state).await,
        Command::Inventory => inventory::inventory(addr, state).await,
        Command::Group(action) => group::group(action, addr, state).await,
        Command::Move { direction } => world::move_player(direction, addr, state).await,
        Command::Attack { target } => combat::attack(target, addr, state).await,
        Command::Talk { target } => world::talk(target, addr, state).await,
        Command::Quest(action) => quest::quest(action, addr, state).await,
        Command::Quit => session::quit(addr, state).await,
        Command::Unknown(raw) => {
            Response::error(404, format!("Unknown command: {}", raw))
        }
    }
}
