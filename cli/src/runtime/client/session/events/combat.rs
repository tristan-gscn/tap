use serde_json::Value;

use super::super::Session;

pub fn handle_combat(session: &mut Session, ev: &str, data: &Value) -> bool {
    let by = data.get("by").and_then(|v| v.as_str()).unwrap_or("?");
    let npc = data.get("npc").and_then(|v| v.as_str()).unwrap_or("?");
    let npc_id = data.get("npc_id").and_then(|v| v.as_u64()).unwrap_or(0);
    match ev {
        "npc_attacked" => {
            let hp = data.get("npc_hp").and_then(|v| v.as_i64()).unwrap_or(0);
            if let Some(n) = session.app.room.npcs.iter_mut().find(|n| n.id == npc_id) {
                n.hp = hp;
            }
            session
                .app
                .logs
                .push(format!("\u{2694} {} hit {} #{} ({} hp)", by, npc, npc_id, hp));
            true
        }
        "npc_killed" => {
            session.app.room.npcs.retain(|n| n.id != npc_id);
            session
                .app
                .logs
                .push(format!("\u{2694} {} killed {} #{}", by, npc, npc_id));
            true
        }
        "player_defeated" => {
            let killer = data.get("by").and_then(|v| v.as_str()).unwrap_or("?");
            session.app.status.hp_current = session.app.status.hp_max;
            session.app.status.combat_status = format!("Defeated by {} — respawned", killer);
            session
                .app
                .logs
                .push(format!("\u{2694} you were defeated by {} and respawned", killer));
            let _ = session.refresh_look();
            true
        }
        _ => false,
    }
}
