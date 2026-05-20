use serde_json::json;

use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

/// Handles the QUEST LIST subcommand, returning all available and active quests.
pub fn list(state: &GameState, name: &str) -> Response {
    let cfg = config::get();
    let player = &state.players[name];

    let quests: Vec<_> = cfg
        .world
        .quests
        .iter()
        .map(|(id, q)| {
            let status = match player.quests.get(id) {
                Some(p) if p.completed => "completed",
                Some(_) => "active",
                None => "available",
            };
            json!({
                "id": id,
                "name": q.name,
                "description": q.description,
                "objective": {
                    "kind": q.objective.kind(),
                    "target": q.objective.target(),
                    "count": q.objective.count(),
                },
                "reward": { "xp": q.reward.xp },
                "status": status,
            })
        })
        .collect();

    Response::ok("quest", json!({ "action": "list", "quests": quests }))
}
