use super::Session;

mod helpers;
mod response;

impl Session {
    pub fn handle_input(&mut self) {
        let trimmed = self.app.input.trim();
        if trimmed.is_empty() {
            self.app.input.clear();
            return;
        }
        let first_word = trimmed.split_whitespace().next().unwrap_or("").to_uppercase();
        let is_command = helpers::is_command(first_word.as_str());

        let to_send = if is_command {
            self.app.logs.push(format!("[Command] > {}", trimmed));
            trimmed.to_string()
        } else {
            self.app.logs.push(format!("[Fallback] > CHAT ROOM {}", trimmed));
            format!("CHAT ROOM {}", trimmed)
        };

        match self.send_command(&to_send) {
            Ok(response) => response::process_response(self, first_word.as_str(), response),
            Err(err) => self.app.logs.push(format!("[Client] Error sending command: {}", err)),
        }

        self.app.input.clear();
    }
}
