use serde_json::Value;

use super::super::super::protocol::{ApiResponse, InventoryResponse, LookResponse};
use super::super::Session;
use super::{chat, combat, group, quest, room};

pub fn handle(session: &mut Session, resp: ApiResponse) {
    match resp {
        ApiResponse::Ok { kind, data } if kind == "event" => handle_event(session, &data),
        ApiResponse::Ok { kind, data } if kind == "look" => {
            if let Ok(look) = serde_json::from_value::<LookResponse>(data) {
                session.apply_look_to_room(look);
            }
        }
        ApiResponse::Ok { kind, data } if kind == "inventory" => {
            if let Ok(inv) = serde_json::from_value::<InventoryResponse>(data) {
                session.apply_inventory_to_status(inv);
            }
        }
        ApiResponse::Error { code, message } => {
            session.app.logs.push(format!("<server> ERR {} {}", code, message));
        }
        _ => {}
    }
}

fn handle_event(session: &mut Session, data: &Value) {
    let ev = data.get("event").and_then(|v| v.as_str()).unwrap_or("");
    if room::handle_room_event(session, ev, data) {
        return;
    }
    if chat::handle_chat(session, ev, data) {
        return;
    }
    if combat::handle_combat(session, ev, data) {
        return;
    }
    if quest::handle_quest(session, ev, data) {
        return;
    }
    group::handle_group(session, ev, data);
}
