use self::events::handle_events;
use self::lines::build_lines;
use super::layout::centered_rect;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::{self, Stdout};

mod events;
mod lines;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Field {
    Name,
    Class,
}

pub fn draw_prompt(stdout: Stdout) -> io::Result<(String, String)> {
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut name = String::new();
    let mut class = "knight".to_string();
    let mut show_error = false;
    let mut active = Field::Name;
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Length(11),
                    Constraint::Percentage(40),
                ])
                .split(area);
            let prompt_area = centered_rect(62, 9, vertical[1]);
            let block = Block::default()
                .title("Connect to Server")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            let lines = build_lines(&name, &class, show_error, active);
            let paragraph = Paragraph::new(lines)
                .block(block)
                .style(Style::default().fg(Color::White));
            frame.render_widget(paragraph, prompt_area);
            let (cursor_x, cursor_y) = match active {
                Field::Name => (prompt_area.x + 8 + name.len() as u16, prompt_area.y + 3),
                Field::Class => (prompt_area.x + 9 + class.len() as u16, prompt_area.y + 5),
            };
            frame.set_cursor(cursor_x, cursor_y);
        })?;
        if handle_events(&mut name, &mut class, &mut show_error, &mut active)? {
            break;
        }
    }
    let name = name.trim().to_string();
    let class = if class.trim().is_empty() {
        "knight".to_string()
    } else {
        class.trim().to_string()
    };
    Ok((name, class))
}
