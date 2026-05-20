mod draw;
mod cleanup;
mod layout;

pub struct ConnectInfo {
    pub name: String,
    pub class: String,
}

/// Prompts the user to enter their player name and class using a temporary TUI.
pub fn prompt_player_info() -> std::io::Result<ConnectInfo> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let result = draw::draw_prompt(stdout);
    let cleanup_result = cleanup::cleanup_terminal();

    match (result, cleanup_result) {
        (Ok((name, class)), Ok(())) => Ok(ConnectInfo { name, class }),
        (Err(err), _) => Err(err),
        (Ok(_), Err(err)) => Err(err),
    }
}

use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
