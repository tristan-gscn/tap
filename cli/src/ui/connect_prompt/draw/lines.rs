use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier, Color};

use super::Field;

pub fn build_lines<'a>(name: &'a str, class: &'a str, show_error: bool, active: Field) -> Vec<Line<'a>> {
    let name_label = if active == Field::Name { "> Name: " } else { "  Name: " };
    let class_label = if active == Field::Class { "> Class: " } else { "  Class: " };
    let mut lines = vec![
        Line::from(Span::styled(
            "Enter your player name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from("") ,
        Line::from(vec![
            Span::styled(name_label, Style::default().fg(Color::Gray)),
            Span::raw(name),
        ]),
        Line::from(vec![
            Span::styled(class_label, Style::default().fg(Color::Gray)),
            Span::raw(class),
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
        "Tab to switch fields. Enter to connect. Esc to cancel.",
        Style::default().fg(Color::DarkGray),
    )));

    lines
}
