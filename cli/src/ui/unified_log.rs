use ratatui::layout::Rect; use ratatui::text::Line; use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::{Frame, style::{Style, Color}}; use crate::app::App;

fn styled(line: &str) -> Line<'_> {
    if let Some(rest) = line.strip_prefix("! ") { return Line::styled(rest, Style::default().fg(Color::Yellow)); }
    if let Some(rest) = line.strip_prefix("[System]") { return Line::styled(rest.trim_start(), Style::default().fg(Color::Yellow)); }
    if line.starts_with("<server>") { return Line::styled(line, Style::default().fg(Color::DarkGray)); }
    if let Some(rest) = line.strip_prefix("[Cmd]") { return Line::styled(rest.trim_start(), Style::default().fg(Color::Cyan)); }
    if line.starts_with('<') { return Line::styled(line, Style::default().fg(Color::White)); }
    Line::from(line)
}

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let max_lines = area.height.saturating_sub(2) as usize;
    let start = app.logs.len().saturating_sub(max_lines);
    let items: Vec<ListItem> = app.logs[start..].iter()
        .map(|l| ListItem::new(styled(l))).collect();
    frame.render_widget(List::new(items).block(Block::default().borders(Borders::ALL)), area);
}