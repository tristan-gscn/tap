use super::Session;

mod helpers;
mod response;

impl Session {
    /// Handles user input validation (Enter key press).
    /// Determines if it's a command or a chat message and sends it to the server.
    pub fn handle_input(&mut self) {
        let trimmed = self.app.input.trim();
        if trimmed.is_empty() {
            self.app.input.clear();
            return;
        }
        let first_word = trimmed
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_uppercase();
        let is_command = helpers::is_command(first_word.as_str());

        // HELP is handled entirely client-side; it is not part of the wire protocol.
        if first_word == "HELP" {
            for line in helpers::help_lines() {
                self.app.logs.push(line);
            }
            self.app.input.clear();
            return;
        }

        let name = self.app.status.name.clone();
        let to_send = if is_command {
            if first_word == "CHAT" {
                if helpers::chat_message(trimmed).is_none() {
                    self.app.logs.push(format!("[Cmd] <{}> {}", name, trimmed));
                }
            } else {
                self.app.logs.push(format!("[Cmd] <{}> {}", name, trimmed));
            }
            trimmed.to_string()
        } else {
            format!("CHAT ROOM {}", trimmed)
        };

        match self.send_command(&to_send) {
            Ok(response) => response::process_response(self, first_word.as_str(), response),
            Err(err) => self
                .app
                .logs
                .push(format!("[Client] Error sending command: {}", err)),
        }

        self.app.input.clear();
    }
}
