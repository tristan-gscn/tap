use std::net::TcpStream;

use crate::app::App;

pub struct Session {
    pub app: App,
    pub stream: TcpStream,
    pub reader: TcpStream,
    pub read_buf: Vec<u8>,
}

mod send;
mod input;
mod connect;
mod reader;
mod events;
mod command;

pub use connect::connect;
