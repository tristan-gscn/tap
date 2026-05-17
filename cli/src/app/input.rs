use super::App;

impl App {
    pub fn submit_input(&mut self) {
        let trimmed = self.input.trim();
        if trimmed.is_empty() {
            self.input.clear();
            return;
        }
        let first_word = trimmed.split_whitespace().next().unwrap_or("").to_uppercase();
        let is_command = matches!(
            first_word.as_str(),
            "CONNECT" | "LOOK" | "MOVE" | "QUIT" | "CHAT" | "WHO" | "GROUP" | "TAKE"
                | "DROP" | "INVENTORY" | "TALK" | "ATTACK" | "STATUS" | "QUEST" | "QUESTS"
        );
        if is_command {
            self.logs.push(format!("[Command] > {}", trimmed));
        } else {
            self.logs.push(format!("[Fallback] > CHAT ROOM {}", trimmed));
        }
        self.input.clear();
    }
}