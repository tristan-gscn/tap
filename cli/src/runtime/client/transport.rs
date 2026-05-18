use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

use super::protocol::ApiResponse;

pub const SERVER_ADDR: &str = "127.0.0.1:4000";

pub fn send_line(stream: &mut TcpStream, line: &str) -> io::Result<()> {
    stream.write_all(line.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()
}

pub fn read_response(stream: &mut TcpStream) -> io::Result<ApiResponse> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    if line.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "empty server response"));
    }

    serde_json::from_str(line.trim()).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}
