use std::io;

use super::types::ApiResponse;

pub fn parse_line(line: &str) -> io::Result<ApiResponse> {
    serde_json::from_str(line).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}
