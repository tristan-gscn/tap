use std::net::TcpStream;

use crate::app::App;

pub struct Session {
    pub app: App,
    pub stream: TcpStream,
}

mod send;
mod input;
mod connect;
mod refresh;

pub use connect::connect;
