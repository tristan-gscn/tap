use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::Field;

pub fn handle_events(
    name: &mut String,
    class: &mut String,
    show_error: &mut bool,
    active: &mut Field,
) -> io::Result<bool> {
    if event::poll(Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            match key.code {
                KeyCode::Enter => {
                    if name.trim().is_empty() {
                        *show_error = true;
                        return Ok(false);
                    } else {
                        return Ok(true);
                    }
                }
                KeyCode::Esc => {
                    name.clear();
                    return Ok(true);
                }
                KeyCode::Tab => {
                    *active = if *active == Field::Name { Field::Class } else { Field::Name };
                }
                KeyCode::Backspace => {
                    match active {
                        Field::Name => {
                            name.pop();
                        }
                        Field::Class => {
                            class.pop();
                        }
                    }
                    *show_error = false;
                }
                KeyCode::Char(c) if !c.is_control() => {
                    match active {
                        Field::Name => name.push(c),
                        Field::Class => class.push(c),
                    }
                    *show_error = false;
                }
                _ => {}
            }
        }
    }
    Ok(false)
}
