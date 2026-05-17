use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::App;
use crate::ui;
use super::keys;

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && keys::handle(key, &mut app) {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}