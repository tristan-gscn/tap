use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);
    let online = Paragraph::new(app.social.online_players.join("\n"))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("ONLINE PLAYERS"),
        )
        .wrap(Wrap { trim: true });
    let group_id = app
        .social
        .group_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "none".to_string());
    let leader = if app.social.group_leader.is_empty() {
        "-"
    } else {
        app.social.group_leader.as_str()
    };
    let members = if app.social.group_members.is_empty() {
        vec![Line::from("(none)")]
    } else {
        app.social
            .group_members
            .iter()
            .map(|m| Line::from(m.as_str()))
            .collect()
    };
    let invites = if app.social.group_invites.is_empty() {
        vec![Line::from("(none)")]
    } else {
        app.social
            .group_invites
            .iter()
            .map(|i| {
                Line::from(format!(
                    "from {} (leader {}, id {})",
                    i.from, i.leader, i.group_id
                ))
            })
            .collect()
    };
    let mut lines = vec![
        Line::from(vec![
            Span::styled("Group: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(group_id),
        ]),
        Line::from(vec![
            Span::styled("Leader: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(leader),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Members:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];
    lines.extend(members);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Invites:",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.extend(invites);
    let group_text = Text::from(lines);
    let group = Paragraph::new(group_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("GROUP MANAGEMENT"),
    );
    frame.render_widget(online, sections[0]);
    frame.render_widget(group, sections[1]);
}
