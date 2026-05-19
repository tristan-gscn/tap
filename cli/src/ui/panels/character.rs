use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Gauge, Paragraph, Wrap}, Frame};
use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let sections = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let hp_ratio = if app.status.hp_max == 0 { 0.0 } else { app.status.hp_current as f64 / app.status.hp_max as f64 };
    let status_text = Text::from(vec![
        Line::from(vec![Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.status.name.as_str())]),
        Line::from(vec![Span::styled("Combat: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.status.combat_status.as_str())]),
        Line::from(vec![Span::styled("XP: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.status.xp.to_string())]),
        Line::from("HP"),
    ]);
    let status_block = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL).title("DETAILED PLAYER STATUS"));
    frame.render_widget(status_block, sections[0]);
    let gauge_area = Rect { x: sections[0].x + 1, y: sections[0].y + 4, width: sections[0].width.saturating_sub(2), height: 1 };
    let gauge = Gauge::default().gauge_style(Style::default().fg(Color::Green)).ratio(hp_ratio)
        .label(format!("{}/{}", app.status.hp_current, app.status.hp_max));
    frame.render_widget(gauge, gauge_area);
    let inventory = Paragraph::new(app.status.inventory.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("INVENTORY"))
        .wrap(Wrap { trim: true });
    let quest_lines = app.status.quests.iter().map(|q| q.label()).collect::<Vec<_>>().join("\n");
    let quests = Paragraph::new(quest_lines)
        .block(Block::default().borders(Borders::ALL).title("QUESTS (tracked + progress)"))
        .wrap(Wrap { trim: true });
    frame.render_widget(inventory, sections[1]);
    frame.render_widget(quests, sections[2]);
}
