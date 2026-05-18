mod draw;
mod cleanup;
mod layout;

pub fn prompt_player_name() -> std::io::Result<String> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let result = draw::draw_prompt(stdout);
    let cleanup_result = cleanup::cleanup_terminal();

    match (result, cleanup_result) {
        (Ok(name), Ok(())) => Ok(name),
        (Err(err), _) => Err(err),
        (Ok(_), Err(err)) => Err(err),
    }
}

use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
