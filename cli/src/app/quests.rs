use serde_json::Value;

use super::mocks::QuestEntry;
use super::App;

impl App {
    fn parse_progress(progress: &str) -> (i64, i64) {
        let mut parts = progress.split('/');
        let current = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        let required = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        (current, required)
    }

    pub fn quest_upsert(&mut self, id: &str, f: impl FnOnce(&mut QuestEntry)) {
        if let Some(q) = self.status.quests.iter_mut().find(|q| q.id == id) {
            f(q);
            return;
        }
        let mut q = QuestEntry {
            id: id.to_string(),
            name: id.to_string(),
            status: "active".to_string(),
            progress: 0,
            required: 0,
        };
        f(&mut q);
        self.status.quests.push(q);
    }

    pub fn apply_quest_list(&mut self, data: &Value) {
        let Some(list) = data.get("quests").and_then(|v| v.as_array()) else {
            return;
        };
        for q in list {
            let Some(id) = q.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            let name = q
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(id)
                .to_string();
            let status = q
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("available")
                .to_string();
            let required = q
                .get("objective")
                .and_then(|o| o.get("count"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            self.quest_upsert(id, |e| {
                e.name = name;
                e.status = status;
                if e.required == 0 {
                    e.required = required;
                }
            });
        }
    }

    pub fn apply_quest_status(&mut self, data: &Value) {
        if let Some(xp) = data.get("xp").and_then(|v| v.as_i64()) {
            self.status.xp = xp;
        }
        let Some(list) = data.get("quests").and_then(|v| v.as_array()) else {
            return;
        };
        for q in list {
            let Some(id) = q.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            let name = q
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(id)
                .to_string();
            let p = q.get("progress").and_then(|v| v.as_i64()).unwrap_or(0);
            let r = q.get("required").and_then(|v| v.as_i64()).unwrap_or(0);
            let done = q.get("completed").and_then(|v| v.as_bool()).unwrap_or(false);
            self.quest_upsert(id, |e| {
                e.name = name;
                e.progress = p;
                e.required = r;
                e.status = if done { "completed" } else { "active" }.to_string();
            });
        }
    }

    pub fn apply_quest_summary(&mut self, data: &Value) {
        let Some(list) = data.get("quests").and_then(|v| v.as_array()) else {
            return;
        };
        for q in list {
            let Some(id) = q
                .get("quest_id")
                .or_else(|| q.get("id"))
                .and_then(|v| v.as_str())
            else {
                continue;
            };
            let status = q
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("active")
                .to_string();
            let (progress, required) = q
                .get("progress")
                .and_then(|v| v.as_str())
                .map(Self::parse_progress)
                .unwrap_or((0, 0));
            self.quest_upsert(id, |e| {
                if e.name == e.id {
                    e.name = id.to_string();
                }
                e.status = status;
                if required > 0 {
                    e.required = required;
                }
                e.progress = progress;
            });
        }
    }

    pub fn apply_quest_request(&mut self, data: &Value) {
        let Some(id) = data
            .get("quest_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
        else {
            return;
        };
        let name = data
            .get("name")
            .and_then(|v| v.as_str())
            .or_else(|| data.get("description").and_then(|v| v.as_str()))
            .unwrap_or(id)
            .to_string();
        let raw_status = data
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("active");
        let status = if raw_status == "available" {
            "active".to_string()
        } else {
            raw_status.to_string()
        };
        self.quest_upsert(id, |e| {
            e.name = name;
            e.status = status;
        });
    }
}
