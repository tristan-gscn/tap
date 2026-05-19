use serde_json::json;

use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub fn status(state: &GameState, name: &str) -> Response {
    let cfg = config::get();
    let player = &state.players[name];

    let active: Vec<_> = player
        .quests
        .iter()
        .filter_map(|(id, prog)| {
            let q = cfg.world.quests.get(id)?;
            Some(json!({
                "id": id,
                "name": q.name,
                "progress": prog.progress,
                "required": q.objective.count(),
                "completed": prog.completed,
            }))
        })
        .collect();

    Response::ok(
        "quest",
        json!({ "action": "status", "quests": active, "xp": player.xp }),
    )
}
