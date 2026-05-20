use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use crate::app::App;

/// Renders the input bar where the user types commands and messages.
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("CHAT OR ACT"));
    frame.render_widget(input, area);
}