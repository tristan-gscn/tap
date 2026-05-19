use serde_json::json;

use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub fn notify_progress(state: &GameState, name: &str, quest_ids: &[String]) {
    let cfg = config::get();
    let Some(player) = state.players.get(name) else {
        return;
    };

    for id in quest_ids {
        let (Some(quest), Some(prog)) = (cfg.world.quests.get(id), player.quests.get(id)) else {
            continue;
        };
        state.send_to(
            name,
            Response::ok(
                "event",
                json!({
                    "event": "quest_progress",
                    "quest": id,
                    "progress": prog.progress,
                    "required": quest.objective.count(),
                    "done": prog.progress >= quest.objective.count(),
                }),
            ),
        );
    }
}
