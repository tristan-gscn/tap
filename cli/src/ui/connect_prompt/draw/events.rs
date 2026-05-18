use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle_events(input: &mut String, show_error: &mut bool) -> io::Result<bool> {
    if event::poll(Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            match key.code {
                KeyCode::Enter => {
                    if input.trim().is_empty() {
                        *show_error = true;
                        return Ok(false);
                    } else {
                        return Ok(true);
                    }
                }
                KeyCode::Esc => {
                    input.clear();
                    return Ok(true);
                }
                KeyCode::Backspace => {
                    input.pop();
                    *show_error = false;
                }
                KeyCode::Char(c) if !c.is_control() => {
                    input.push(c);
                    *show_error = false;
                }
                _ => {}
            }
        }
    }
    Ok(false)
}
