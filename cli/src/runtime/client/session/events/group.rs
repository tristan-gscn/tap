use serde_json::Value;

use super::super::Session;
use crate::app::mocks::GroupInvite;

pub fn handle_group(session: &mut Session, ev: &str, data: &Value) -> bool {
    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
    match ev {
        "group_invite" => {
            let leader = data.get("leader").and_then(|v| v.as_str()).unwrap_or("?");
            let from = data.get("from").and_then(|v| v.as_str()).unwrap_or(leader);
            let gid = data.get("group").and_then(|v| v.as_u64()).unwrap_or(0);
            if gid > 0
                && !session
                    .app
                    .social
                    .group_invites
                    .iter()
                    .any(|i| i.group_id == gid)
            {
                session.app.social.group_invites.push(GroupInvite {
                    group_id: gid,
                    leader: leader.to_string(),
                    from: from.to_string(),
                });
            }
            session.app.logs.push(format!(
                "! group invite from {} (leader {}, id {})",
                from, leader, gid
            ));
            true
        }
        "group_join" if !name.is_empty() => {
            if !session.app.social.group_members.iter().any(|m| m == name) {
                session.app.social.group_members.push(name.to_string());
            }
            let leader = session.app.social.group_leader.clone();
            if !leader.is_empty()
                && !session
                    .app
                    .social
                    .group_members
                    .iter()
                    .any(|m| m == &leader)
            {
                session.app.social.group_members.insert(0, leader);
            }
            session
                .app
                .logs
                .push(format!("! {} joined the group", name));
            true
        }
        "group_leave" if !name.is_empty() => {
            session.app.social.group_members.retain(|m| m != name);
            session.app.logs.push(format!("! {} left the group", name));
            true
        }
        "group_disband" => {
            session.app.social.group_id = None;
            session.app.social.group_leader.clear();
            session.app.social.group_members.clear();
            session.app.social.group_invites.clear();
            session.app.logs.push("! group disbanded".to_string());
            true
        }
        _ => false,
    }
}
