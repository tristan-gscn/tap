use super::super::Session;

pub fn handle_group(session: &mut Session, ev: &str) -> bool {
    let msg = match ev {
        "group_invite" => Some("group invite received"),
        "group_join" => Some("player joined the group"),
        "group_leave" => Some("player left the group"),
        "group_disband" => Some("group disbanded"),
        _ => None,
    };
    if let Some(m) = msg {
        session.app.logs.push(format!("! {}", m));
        return true;
    }
    false
}
