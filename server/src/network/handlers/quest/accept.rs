use serde_json::json;
use tracing::info;

use super::resolve_quest;
use crate::protocol::response::Response;
use crate::state::game::GameState;
use crate::state::player::QuestProgress;

/// Handles the QUEST ACCEPT subcommand, accepting a quest by ID.
pub fn accept(state: &mut GameState, name: &str, id: String) -> Response {
    let qid = match resolve_quest(&id) {
        Some(q) => q,
        None => return Response::error(404, "QUEST_NOT_FOUND"),
    };
    if state.players[name].quests.contains_key(&qid) {
        return Response::error(402, "QUEST_ALREADY_ACCEPTED");
    }

    if let Some(p) = state.players.get_mut(name) {
        p.quests.insert(qid.clone(), QuestProgress::default());
    }

    state.send_to(
        name,
        Response::ok("event", json!({ "event": "quest_accepted", "quest": qid })),
    );

    info!(player = %name, quest = %qid, "Quest accepted");
    Response::ok("quest", json!({ "action": "accept", "quest": qid }))
}
