use std::io::{self, Stdout};
use ratatui::backend::CrosstermBackend; use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout}; use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph}; use super::layout::centered_rect;
use self::events::handle_events; use self::lines::build_lines;

mod events;
mod lines;

pub fn draw_prompt(stdout: Stdout) -> io::Result<String> {
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input = String::new(); let mut show_error = false;
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let vertical = Layout::default().direction(Direction::Vertical).constraints([
                Constraint::Percentage(40), Constraint::Length(11), Constraint::Percentage(40),
            ]).split(area);
            let prompt_area = centered_rect(62, 9, vertical[1]);
            let block = Block::default().title("Connect to Server").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan));
            let visible_name = input.as_str();
            let lines = build_lines(visible_name, show_error);
            let paragraph = Paragraph::new(lines).block(block).style(Style::default().fg(Color::White));
            frame.render_widget(paragraph, prompt_area);
            let cursor_x = prompt_area.x + 7 + input.len() as u16; let cursor_y = prompt_area.y + 3; frame.set_cursor(cursor_x, cursor_y);
        })?;
        if handle_events(&mut input, &mut show_error)? { break; }
    }
    Ok(input.trim().to_string())
}
