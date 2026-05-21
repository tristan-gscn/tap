use std::net::TcpStream;

use crate::app::App;

pub struct Session {
    pub app: App,
    pub stream: TcpStream,
    pub reader: TcpStream,
    pub read_buf: Vec<u8>,
}

mod command;
mod connect;
mod events;
mod input;
mod reader;
mod send;

pub use connect::connect;
