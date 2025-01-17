use std::{
    error::Error,
    io::{self, Stderr, Write},
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};

pub fn init_terminal<B: Backend>() -> Result<Terminal<CrosstermBackend<Stderr>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn cleanup_terminal<B: Backend + Write>(
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    terminal.show_cursor()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}
