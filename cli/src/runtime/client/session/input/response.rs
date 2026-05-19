use serde_json::Value;

use super::super::super::protocol::{parse_inventory, parse_look, ApiResponse};
use super::super::Session;

pub fn process_response(session: &mut Session, first_word: &str, response: ApiResponse) {
    match first_word {
        "LOOK" => match parse_look(response) {
            Ok(look) => session.apply_look_to_room(look),
            Err(err) => session
                .app
                .logs
                .push(format!("[Client] Error updating room after LOOK: {}", err)),
        },
        "INVENTORY" => match parse_inventory(response) {
            Ok(inv) => session.apply_inventory_to_status(inv),
            Err(err) => session
                .app
                .logs
                .push(format!("[Client] Error updating inventory after INVENTORY: {}", err)),
        },
        "MOVE" | "GO" => {
            if let ApiResponse::Ok { data, .. } = &response {
                let dir = data.get("direction").and_then(|v| v.as_str()).unwrap_or("?");
                let to = data.get("to").and_then(|v| v.as_str()).unwrap_or("?");
                session
                    .app
                    .logs
                    .push(format!("\u{2192} moved {} to {}", dir, to));
            }
            if let ApiResponse::Ok { .. } = response {
                let _ = session.refresh_look();
            }
        }
        "ATTACK" => {
            if let ApiResponse::Ok { data, .. } = &response {
                apply_attack(session, data);
            }
        }
        "QUEST" | "QUESTS" => {
            if let ApiResponse::Ok { data, .. } = &response {
                apply_quest(session, data);
            }
        }
        "WHO" => {
            if let ApiResponse::Ok { data, .. } = &response {
                apply_who(session, data);
            }
        }
        "GROUP" => {
            if let ApiResponse::Ok { data, .. } = &response {
                apply_group(session, data);
            }
        }
        _ => {}
    }
}

fn as_hp(v: Option<&Value>) -> Option<u16> {
    v.and_then(|x| x.as_i64()).map(|n| n.max(0) as u16)
}

fn apply_attack(session: &mut Session, data: &Value) {
    let target = data.get("target").and_then(|v| v.as_str()).unwrap_or("?");
    let defeated = data.get("defeated").and_then(|v| v.as_bool()).unwrap_or(false);
    let killed = data.get("killed").and_then(|v| v.as_bool()).unwrap_or(false);

    if defeated {
        session.app.status.hp_current = session.app.status.hp_max;
        session.app.status.combat_status = "Defeated — respawned".to_string();
        let _ = session.refresh_look();
        return;
    }
    if let Some(hp) = as_hp(data.get("player_hp")) {
        session.app.status.hp_current = hp;
    }
    if killed {
        session.app.status.combat_status = "Calm".to_string();
    } else if let Some(npc_hp) = data.get("npc_hp").and_then(|v| v.as_i64()) {
        session.app.status.combat_status = format!("Fighting {} ({} hp)", target, npc_hp);
    }
}

fn apply_quest(session: &mut Session, data: &Value) {
    match data.get("action").and_then(|v| v.as_str()).unwrap_or("") {
        "list" => session.app.apply_quest_list(data),
        "summary" => session.app.apply_quest_summary(data),
        "request" => session.app.apply_quest_request(data),
        "status" => session.app.apply_quest_status(data),
        "complete" => {
            if let Some(total) = data.get("total_xp").and_then(|v| v.as_i64()) {
                session.app.status.xp = total;
            }
        }
        _ => {}
    }
}

fn apply_who(session: &mut Session, data: &Value) {
    if let Some(list) = data.get("players").and_then(|v| v.as_array()) {
        session.app.social.online_players = list
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }
}

fn apply_group(session: &mut Session, data: &Value) {
    let action = data.get("action").and_then(|v| v.as_str()).unwrap_or("");
    let me = session.app.status.name.clone();
    match action {
        "create" => {
            session.app.social.group_leader = me.clone();
            session.app.social.group_members = vec![me];
        }
        "leave" => {
            session.app.social.group_leader.clear();
            session.app.social.group_members.clear();
        }
        _ => {}
    }
}
