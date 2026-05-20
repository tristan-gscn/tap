use serde_json::json;
use tracing::info;

use crate::config;
use crate::network::handlers::resolve_npc;
use crate::protocol::response::Response;
use crate::state::game::GameState;
use crate::state::player::QuestProgress;

/// Handles the QUEST <NPC> command, requesting a quest from an NPC.
pub fn request(state: &mut GameState, name: &str, npc_query: String) -> Response {
    let room = state.players[name].room.clone();
    let npc_type = match resolve_npc(&npc_query) {
        Some(t) => t,
        None => return Response::error(404, "NPC_NOT_FOUND"),
    };

    let present = state
        .world
        .npcs_in(&room)
        .iter()
        .any(|n| n.npc_type == npc_type);
    if !present {
        return Response::error(404, "NPC_NOT_FOUND");
    }

    let cfg = config::get();
    let npc = match cfg.world.npcs.get(&npc_type) {
        Some(n) => n,
        None => return Response::error(404, "NPC_NOT_FOUND"),
    };

    let mut selected = None;
    for qid in &npc.quests {
        let Some(quest) = cfg.world.quests.get(qid) else {
            continue;
        };
        match state.players[name].quests.get(qid) {
            Some(p) if p.completed => continue,
            Some(_) => continue,
            None => {
                selected = Some((qid.clone(), quest));
                break;
            }
        }
    }

    let (quest_id, quest) = match selected {
        Some(v) => v,
        None => return Response::error(406, "NO_QUEST_AVAILABLE"),
    };

    if let Some(p) = state.players.get_mut(name) {
        p.quests
            .insert(quest_id.clone(), QuestProgress::default());
    }

    state.send_to(
        name,
        Response::ok(
            "event",
            json!({ "event": "quest_accepted", "quest": quest_id }),
        ),
    );

    info!(player = %name, npc = %npc_type, quest = %quest_id, "Quest requested");
    Response::ok(
        "quest",
        json!({
            "action": "request",
            "quest_id": quest_id,
            "name": quest.name,
            "description": quest.description,
            "reward": format!("xp:{}", quest.reward.xp),
            "status": "available",
        }),
    )
}
