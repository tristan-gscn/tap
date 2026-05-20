use std::io;

use super::super::protocol::ApiResponse;
use super::reader::read_line_blocking;
use super::Session;

impl Session {
    /// Sends a command to the server and waits for the response.
    /// Handles asynchronous events received while waiting for the response.
    pub fn send_command(&mut self, line: &str) -> io::Result<ApiResponse> {
        super::super::transport::send_line(&mut self.stream, line)?;
        loop {
            let raw = read_line_blocking(&mut self.reader, &mut self.read_buf)?;
            let resp = super::super::protocol::parse_line(&raw)?;
            match &resp {
                ApiResponse::Ok { kind, .. } if kind == "event" => {
                    super::events::apply_event(self, resp);
                }
                _ => {
                    self.log_response(&resp);
                    return Ok(resp);
                }
            }
        }
    }
}
