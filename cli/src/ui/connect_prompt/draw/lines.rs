use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier, Color};

pub fn build_lines(visible_name: &str, show_error: bool) -> Vec<Line<'_>> {
    let mut lines = vec![
        Line::from(Span::styled(
            "Enter your player name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from("") ,
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

    lines
}
