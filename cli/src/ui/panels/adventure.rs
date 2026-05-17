use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph, Wrap}, Frame};
use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let sections = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(1)]).split(area);
    let info_text = Text::from(vec![
        Line::from(vec![Span::styled("Room: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.room.name.as_str())]),
        Line::from(vec![Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.room.description.as_str())]),
        Line::from(vec![Span::styled("Exits: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.room.exits.join(", "))]),
    ]);
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("ROOM INFO & EXITS"))
        .wrap(Wrap { trim: true });
    let entity_chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).split(sections[1]);
    let players = Paragraph::new(app.room.players.join("\n")).block(Block::default().borders(Borders::ALL).title("PLAYERS"));
    let npcs = Paragraph::new(app.room.npcs.join("\n")).block(Block::default().borders(Borders::ALL).title("NPCS"));
    let items = Paragraph::new(app.room.items.join("\n")).block(Block::default().borders(Borders::ALL).title("ITEMS"));
    frame.render_widget(info, sections[0]);
    frame.render_widget(players, entity_chunks[0]);
    frame.render_widget(npcs, entity_chunks[1]);
    frame.render_widget(items, entity_chunks[2]);
}