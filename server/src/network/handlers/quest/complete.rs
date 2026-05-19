use serde_json::json;
use tracing::info;

use super::resolve_quest;
use crate::config;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub fn complete(state: &mut GameState, name: &str, id: String) -> Response {
    let qid = match resolve_quest(&id) {
        Some(q) => q,
        None => return Response::error(404, "QUEST_NOT_FOUND"),
    };

    let cfg = config::get();
    let quest = match cfg.world.quests.get(&qid) {
        Some(q) => q,
        None => return Response::error(404, "QUEST_NOT_FOUND"),
    };

    let prog = match state.players[name].quests.get(&qid) {
        Some(p) => p.clone(),
        None => return Response::error(401, "QUEST_NOT_ACCEPTED"),
    };
    if prog.completed {
        return Response::error(402, "QUEST_ALREADY_COMPLETED");
    }
    if prog.progress < quest.objective.count() {
        return Response::error(
            409,
            format!(
                "QUEST_INCOMPLETE ({}/{})",
                prog.progress,
                quest.objective.count()
            ),
        );
    }

    let reward = quest.reward.xp;
    let mut total_xp = 0;
    if let Some(p) = state.players.get_mut(name) {
        if let Some(pr) = p.quests.get_mut(&qid) {
            pr.completed = true;
        }
        p.xp += reward;
        total_xp = p.xp;
    }

    state.send_to(
        name,
        Response::ok(
            "event",
            json!({
                "event": "quest_completed",
                "quest": qid,
                "reward_xp": reward,
                "total_xp": total_xp,
            }),
        ),
    );

    info!(player = %name, quest = %qid, reward, "Quest completed");
    Response::ok(
        "quest",
        json!({
            "action": "complete",
            "quest": qid,
            "reward_xp": reward,
            "total_xp": total_xp,
        }),
    )
}
