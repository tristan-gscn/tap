use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::LeaveAlternateScreen;
use std::io;

pub fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
