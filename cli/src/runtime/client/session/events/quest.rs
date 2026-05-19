use serde_json::Value;

use super::super::Session;

pub fn handle_quest(session: &mut Session, ev: &str, data: &Value) -> bool {
    let quest = data.get("quest").and_then(|v| v.as_str()).unwrap_or("");
    if quest.is_empty() {
        return false;
    }
    match ev {
        "quest_accepted" => {
            session.app.quest_upsert(quest, |e| e.status = "active".to_string());
            session
                .app
                .logs
                .push(format!("\u{2726} quest accepted: {}", quest));
            true
        }
        "quest_progress" => {
            let p = data.get("progress").and_then(|v| v.as_i64()).unwrap_or(0);
            let r = data.get("required").and_then(|v| v.as_i64()).unwrap_or(0);
            let done = data.get("done").and_then(|v| v.as_bool()).unwrap_or(false);
            session.app.quest_upsert(quest, |e| {
                e.progress = p;
                e.required = r;
                e.status = if done { "ready" } else { "active" }.to_string();
            });
            session
                .app
                .logs
                .push(format!("\u{2726} quest {} progress {}/{}", quest, p, r));
            true
        }
        "quest_completed" => {
            let reward = data.get("reward_xp").and_then(|v| v.as_i64()).unwrap_or(0);
            let total = data.get("total_xp").and_then(|v| v.as_i64()).unwrap_or(0);
            session
                .app
                .quest_upsert(quest, |e| e.status = "completed".to_string());
            session.app.status.xp = total;
            session
                .app
                .logs
                .push(format!("\u{2726} quest completed: {} (+{} xp)", quest, reward));
            true
        }
        _ => false,
    }
}
