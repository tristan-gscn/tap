use std::io;
use super::reader::read_line_blocking; use super::Session;

impl Session {
    pub fn send_command(&mut self, line: &str) -> io::Result<super::super::protocol::ApiResponse> {
        super::super::transport::send_line(&mut self.stream, line)?;
        let line = read_line_blocking(&mut self.reader, &mut self.read_buf)?;
        let resp = super::super::protocol::parse_line(&line)?;
        self.log_response(&resp);
        Ok(resp)
    }
}
