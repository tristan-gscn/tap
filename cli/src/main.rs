use std::io;

mod app;
mod runtime;
mod ui;

fn main() -> io::Result<()> {
    runtime::start()
}
