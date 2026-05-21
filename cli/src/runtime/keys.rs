use super::client::Session;
use crate::app::Tab;
use crossterm::event::{KeyCode, KeyEvent};

/// Handles a keyboard key press.
/// Returns true if the application should exit.
pub fn handle(key: KeyEvent, session: &mut Session) -> bool {
    match key.code {
        KeyCode::Esc => return true,
        KeyCode::F(1) => session.app.current_tab = Tab::Adventure,
        KeyCode::F(2) => session.app.current_tab = Tab::Character,
        KeyCode::F(3) => session.app.current_tab = Tab::Social,
        KeyCode::Enter => session.handle_input(),
        KeyCode::Backspace => {
            session.app.input.pop();
        }
        KeyCode::Char(c) => {
            session.app.input.push(c);
        }
        _ => {}
    }
    false
}
