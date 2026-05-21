use std::io;

use super::reader::read_lines;
use super::Session;

mod chat;
mod combat;
mod group;
mod handle;
mod quest;
mod room;

pub use handle::handle as apply_event;

impl Session {
    /// Retrieves and processes pending events from the network stream.
    pub fn poll_events(&mut self) -> io::Result<()> {
        let lines = read_lines(&mut self.reader, &mut self.read_buf)?;
        for line in lines {
            match super::super::protocol::parse_line(&line) {
                Ok(resp) => handle::handle(self, resp),
                Err(err) => self
                    .app
                    .logs
                    .push(format!("[Client] Bad event line: {}", err)),
            }
        }
        Ok(())
    }
}
