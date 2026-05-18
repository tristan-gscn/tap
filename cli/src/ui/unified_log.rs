use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;
use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let max_lines = area.height.saturating_sub(2) as usize;
    let start = app.logs.len().saturating_sub(max_lines);
    let items: Vec<ListItem> = app.logs[start..].iter()
        .map(|line| ListItem::new(Line::from(line.as_str()))).collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL));
    frame.render_widget(list, area);
}