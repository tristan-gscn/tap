use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

pub fn prompt_player_name() -> io::Result<String> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let result = draw_prompt(stdout);
    let cleanup_result = cleanup_terminal();

    match (result, cleanup_result) {
        (Ok(name), Ok(())) => Ok(name),
        (Err(err), _) => Err(err),
        (Ok(_), Err(err)) => Err(err),
    }
}

fn draw_prompt(stdout: Stdout) -> io::Result<String> {
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input = String::new();
    let mut show_error = false;

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

            let visible_name = input.as_str();

            let mut lines = vec![
                Line::from(Span::styled(
                    "Enter your player name",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Gray)),
                    Span::raw(visible_name),
                ]),
                Line::from("")];

            if show_error {
                lines.push(Line::from(Span::styled(
                    "Name cannot be empty",
                    Style::default().fg(Color::Red),
                )));
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(
                "Press Enter to connect, or Esc to use the default name",
                Style::default().fg(Color::DarkGray),
            )));

            let paragraph = Paragraph::new(lines)
                .block(block)
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, prompt_area);

            let cursor_x = prompt_area.x + 7 + input.len() as u16;
            let cursor_y = prompt_area.y + 3;
            frame.set_cursor(cursor_x, cursor_y);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Enter => {
                        if input.trim().is_empty() {
                            show_error = true;
                            continue;
                        } else {
                            break;
                        }
                    }
                    KeyCode::Esc => {
                        input.clear();
                        break;
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        show_error = false;
                    }
                    KeyCode::Char(c) if !c.is_control() => {
                        input.push(c);
                        show_error = false;
                    }
                    _ => {}
                }
            }
        }
    }

    let name = input.trim();
    Ok(name.to_string())
}

fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Length(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}