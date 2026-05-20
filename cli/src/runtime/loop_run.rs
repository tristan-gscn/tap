use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::ui;
use super::keys;
use super::client::Session;

/// Runs the main application loop (rendering, network polling, keyboard input).
pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, session: &mut Session) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &session.app))?;
        if let Err(err) = session.poll_events() {
            session.app.logs.push(format!("[Client] Event poll error: {}", err));
        }
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && keys::handle(key, session) {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}