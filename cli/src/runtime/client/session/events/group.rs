use super::super::Session;

pub fn handle_group(session: &mut Session, ev: &str) -> bool {
    if matches!(ev, "group_invite" | "group_join" | "group_leave" | "group_disband") {
        session.app.logs.push(format!("[Group] {}", ev));
        return true;
    }
    false
}
