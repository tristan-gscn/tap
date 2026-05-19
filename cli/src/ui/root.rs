use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;
use crate::app::App;
use super::{input_bar, main_area};

pub fn draw(frame: &mut Frame, app: &App) {
    let root = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)]).split(frame.size());
    main_area::draw(frame, root[0], app);
    input_bar::draw(frame, root[1], app);
}