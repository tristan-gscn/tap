use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use crate::app::App;
use super::{right_panel, unified_log};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    unified_log::draw(frame, chunks[0], app);
    right_panel::draw(frame, chunks[1], app);
}