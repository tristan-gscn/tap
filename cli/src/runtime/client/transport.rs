use std::io::{self, Write};
use std::net::TcpStream;

pub const SERVER_ADDR: &str = "127.0.0.1:4000";

pub fn send_line(stream: &mut TcpStream, line: &str) -> io::Result<()> {
    stream.write_all(line.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()
}
