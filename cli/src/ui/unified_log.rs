use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::{style::{Color, Style}, Frame};
use crate::app::App;

/// Classifies a log line for styling based on its prefix.
fn classify(line: &str) -> (Style, &str) {
    if let Some(r) = line.strip_prefix("\u{2694} ") {
        return (Style::default().fg(Color::Red), r);
    }
    if let Some(r) = line.strip_prefix("\u{2726} ") {
        return (Style::default().fg(Color::Magenta), r);
    }
    if let Some(r) = line.strip_prefix("\u{2192} ") {
        return (Style::default().fg(Color::Cyan), r);
    }
    if let Some(r) = line.strip_prefix("! ") {
        return (Style::default().fg(Color::Yellow), r);
    }
    if let Some(r) = line.strip_prefix("[System]") {
        return (Style::default().fg(Color::Yellow), r.trim_start());
    }
    if line.starts_with("<server>") {
        return (Style::default().fg(Color::DarkGray), line);
    }
    if let Some(r) = line.strip_prefix("[Cmd]") {
        return (Style::default().fg(Color::Cyan), r.trim_start());
    }
    if line.starts_with('<') {
        return (Style::default().fg(Color::White), line);
    }
    (Style::default(), line)
}

/// Splits a string into two parts at the given character index.
fn take_chars(s: &str, n: usize) -> (String, String) {
    let mut it = s.chars();
    let head: String = it.by_ref().take(n).collect();
    (head, it.collect())
}

/// Simple word-wrapping for strings based on a maximum width.
fn wrap(s: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![s.to_string()];
    }
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in s.split_whitespace() {
        let wlen = word.chars().count();
        if wlen > width {
            if !cur.is_empty() {
                lines.push(std::mem::take(&mut cur));
            }
            let mut rest = word.to_string();
            while rest.chars().count() > width {
                let (head, tail) = take_chars(&rest, width);
                lines.push(head);
                rest = tail;
            }
            cur = rest;
            continue;
        }
        let cur_len = cur.chars().count();
        let need = if cur_len == 0 { wlen } else { cur_len + 1 + wlen };
        if need > width {
            lines.push(std::mem::take(&mut cur));
            cur = word.to_string();
        } else {
            if !cur.is_empty() {
                cur.push(' ');
            }
            cur.push_str(word);
        }
    }
    if !cur.is_empty() || lines.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Renders the unified log area showing system messages, chat, and events.
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let width = area.width.saturating_sub(2) as usize;
    let max_lines = area.height.saturating_sub(2) as usize;

    let mut all: Vec<Line> = Vec::new();
    for raw in &app.logs {
        let (style, content) = classify(raw);
        for piece in wrap(content, width) {
            all.push(Line::styled(piece, style));
        }
    }

    let start = all.len().saturating_sub(max_lines);
    let mut items: Vec<ListItem> = all[start..]
        .iter()
        .cloned()
        .map(ListItem::new)
        .collect();
    if items.len() < max_lines {
        let pad = max_lines - items.len();
        let mut padded = vec![ListItem::new(Line::from("")); pad];
        padded.append(&mut items);
        items = padded;
    }
    frame.render_widget(
        List::new(items).block(Block::default().borders(Borders::ALL)),
        area,
    );
}
