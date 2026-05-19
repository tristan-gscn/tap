use std::io::{self, Read};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

pub fn read_lines(reader: &mut TcpStream, buf: &mut Vec<u8>) -> io::Result<Vec<String>> {
    let mut tmp = [0u8; 1024];
    loop {
        match reader.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => buf.extend_from_slice(&tmp[..n]),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e),
        }
    }

    let mut lines = Vec::new();
    while let Some(pos) = buf.iter().position(|b| *b == b'\n') {
        let bytes: Vec<u8> = buf.drain(..=pos).collect();
        let line = String::from_utf8_lossy(&bytes).trim().to_string();
        if !line.is_empty() {
            lines.push(line);
        }
    }
    Ok(lines)
}

pub fn read_line_blocking(reader: &mut TcpStream, buf: &mut Vec<u8>) -> io::Result<String> {
    let mut tmp = [0u8; 1024];
    loop {
        while let Some(pos) = buf.iter().position(|b| *b == b'\n') {
            let bytes: Vec<u8> = buf.drain(..=pos).collect();
            let line = String::from_utf8_lossy(&bytes).trim().to_string();
            if !line.is_empty() {
                return Ok(line);
            }
        }
        match reader.read(&mut tmp) {
            Ok(0) => sleep(Duration::from_millis(10)),
            Ok(n) => buf.extend_from_slice(&tmp[..n]),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(10))
            }
            Err(e) => return Err(e),
        }
    }
}
