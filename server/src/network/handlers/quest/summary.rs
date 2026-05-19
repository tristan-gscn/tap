use serde_json::{json, Value};

use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub fn summary(state: &GameState, name: &str) -> Response {
    let cfg = config::get();
    let player = &state.players[name];

    let quests: Vec<Value> = player
        .quests
        .iter()
        .filter_map(|(id, prog)| {
            let quest = cfg.world.quests.get(id)?;
            let status = if prog.completed { "completed" } else { "active" };
            let mut entry = json!({
                "quest_id": id,
                "status": status,
            });
            if !prog.completed {
                entry["progress"] = json!(format!(
                    "{}/{}",
                    prog.progress,
                    quest.objective.count()
                ));
            }
            Some(entry)
        })
        .collect();

    Response::ok("quest", json!({ "action": "summary", "quests": quests }))
}
