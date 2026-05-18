use std::io;

use super::reader::read_lines;
use super::Session;

mod handle;
mod room;
mod chat;
mod group;

impl Session {
    pub fn poll_events(&mut self) -> io::Result<()> {
        let lines = read_lines(&mut self.reader, &mut self.read_buf)?;
        for line in lines {
            match super::super::protocol::parse_line(&line) {
                Ok(resp) => handle::handle(self, resp),
                Err(err) => self.app.logs.push(format!("[Client] Bad event line: {}", err)),
            }
        }
        Ok(())
    }
}
