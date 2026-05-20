use std::io;

mod app;
mod runtime;
mod ui;

/// Main entry point for the CLI binary.
/// Initializes and runs the application.
fn main() -> io::Result<()> {
    runtime::start()
}
