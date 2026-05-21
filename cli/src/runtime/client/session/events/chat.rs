use super::super::Session;
use serde_json::Value;

pub fn handle_chat(session: &mut Session, ev: &str, data: &Value) -> bool {
    if !matches!(ev, "chat_room" | "chat_group" | "chat_global") {
        return false;
    }
    let from = data.get("from").and_then(|v| v.as_str()).unwrap_or("?");
    let text = data.get("text").and_then(|v| v.as_str()).unwrap_or("");
    session.app.logs.push(format!("<{}> {}", from, text));
    true
}
