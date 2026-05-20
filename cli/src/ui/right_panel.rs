use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::Frame;
use crate::app::{App, Tab};
use super::panels;

/// Renders the right panel containing tabs for different game views.
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let tabs = Tabs::new(vec![Line::from("[F1] Adventure"), Line::from("[F2] Character"), Line::from("[F3] Social")])
        .select(match app.current_tab { Tab::Adventure => 0, Tab::Character => 1, Tab::Social => 2 })
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)]).split(area);
    frame.render_widget(tabs, chunks[0]);
    match app.current_tab {
        Tab::Adventure => panels::adventure::draw(frame, chunks[1], app),
        Tab::Character => panels::character::draw(frame, chunks[1], app),
        Tab::Social => panels::social::draw(frame, chunks[1], app),
    }
}