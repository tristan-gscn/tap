use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, Tab};

pub fn handle(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Esc => return true,
        KeyCode::F(1) => app.current_tab = Tab::Adventure,
        KeyCode::F(2) => app.current_tab = Tab::Character,
        KeyCode::F(3) => app.current_tab = Tab::Social,
        KeyCode::Enter => app.submit_input(),
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        _ => {}
    }
    false
}