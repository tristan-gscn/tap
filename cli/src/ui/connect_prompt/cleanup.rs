use std::io;
use crossterm::terminal::disable_raw_mode;
use crossterm::execute;
use crossterm::terminal::LeaveAlternateScreen;

pub fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
