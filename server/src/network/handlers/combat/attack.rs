use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::network::handlers::quest::notify_progress;
use crate::network::handlers::resolve_npc;
use crate::protocol::response::Response;
use crate::state::game::GameState;
use crate::state::world::AttackOutcome;

/// Handles the ATTACK command, allowing a player to damage an NPC in their room.
pub async fn attack(query: String, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room = state.players[&name].room.clone();
    let power = state.players[&name].attack;

    let npc_type = match resolve_npc(&query) {
        Some(t) => t,
        None => return Response::error(404, "NPC_NOT_FOUND"),
    };

    match state.world.attack_npc(&room, &npc_type, power) {
        AttackOutcome::NoTarget => Response::error(404, "NPC_NOT_FOUND"),

        AttackOutcome::Hit {
            npc_id,
            npc_type,
            npc_hp,
            npc_attack,
        } => {
            let mut player_hp = 0;
            let mut defeated = false;
            if let Some(p) = state.players.get_mut(&name) {
                p.hp -= npc_attack.max(0);
                player_hp = p.hp;
                if p.hp <= 0 {
                    p.respawn();
                    defeated = true;
                }
            }

            state.broadcast_room(
                &room,
                None,
                Response::ok(
                    "event",
                    json!({
                        "event": "npc_attacked",
                        "by": name,
                        "npc": npc_type,
                        "npc_id": npc_id,
                        "npc_hp": npc_hp,
                    }),
                ),
            );

            if defeated {
                state.send_to(
                    &name,
                    Response::ok(
                        "event",
                        json!({ "event": "player_defeated", "by": npc_type }),
                    ),
                );
                info!(player = %name, npc = %npc_type, "Player defeated, respawned");
                return Response::ok(
                    "attack",
                    json!({
                        "target": npc_type,
                        "npc_id": npc_id,
                        "npc_hp": npc_hp,
                        "player_hp": 0,
                        "defeated": true,
                    }),
                );
            }

            info!(player = %name, npc = %npc_type, npc_hp, "Attack landed");
            Response::ok(
                "attack",
                json!({
                    "target": npc_type,
                    "npc_id": npc_id,
                    "npc_hp": npc_hp,
                    "player_hp": player_hp,
                    "killed": false,
                }),
            )
        }

        AttackOutcome::Killed { npc_id, npc_type } => {
            let touched = state
                .players
                .get_mut(&name)
                .map(|p| p.advance_quests("kill", &npc_type, 1))
                .unwrap_or_default();

            state.broadcast_room(
                &room,
                None,
                Response::ok(
                    "event",
                    json!({
                        "event": "npc_killed",
                        "by": name,
                        "npc": npc_type,
                        "npc_id": npc_id,
                    }),
                ),
            );

            notify_progress(&state, &name, &touched);

            info!(player = %name, npc = %npc_type, "NPC killed");
            Response::ok(
                "attack",
                json!({
                    "target": npc_type,
                    "npc_id": npc_id,
                    "killed": true,
                }),
            )
        }
    }
}
