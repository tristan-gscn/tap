use serde_json::Value;

use super::super::Session;

pub fn handle_group(session: &mut Session, ev: &str, data: &Value) -> bool {
    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
    match ev {
        "group_invite" => {
            if let Some(leader) = data.get("leader").and_then(|v| v.as_str()) {
                session.app.social.group_leader = leader.to_string();
            }
            session.app.logs.push("! group invite received".to_string());
            true
        }
        "group_join" if !name.is_empty() => {
            if !session.app.social.group_members.iter().any(|m| m == name) {
                session.app.social.group_members.push(name.to_string());
            }
            session.app.logs.push(format!("! {} joined the group", name));
            true
        }
        "group_leave" if !name.is_empty() => {
            session.app.social.group_members.retain(|m| m != name);
            session.app.logs.push(format!("! {} left the group", name));
            true
        }
        "group_disband" => {
            session.app.social.group_leader.clear();
            session.app.social.group_members.clear();
            session.app.logs.push("! group disbanded".to_string());
            true
        }
        _ => false,
    }
}
