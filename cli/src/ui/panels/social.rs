use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph, Wrap}, Frame};
use crate::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let sections = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);
    let online = Paragraph::new(app.social.online_players.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("ONLINE PLAYERS"))
        .wrap(Wrap { trim: true });
    let group_text = Text::from(vec![
        Line::from(vec![Span::styled("Leader: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(app.social.group_leader.as_str())]),
        Line::from(""),
        Line::from(Span::styled("Members:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(app.social.group_members.join("\n")),
    ]);
    let group = Paragraph::new(group_text).block(Block::default().borders(Borders::ALL).title("GROUP MANAGEMENT"));
    frame.render_widget(online, sections[0]);
    frame.render_widget(group, sections[1]);
}