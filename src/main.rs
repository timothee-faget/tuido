use std::{error::Error, io};

use app::{App, ScreenMode, SwitchProjectsDirection, TaskNavDirection};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::build()?;
    let _res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // app.projects[0].add_task(1, String::from("Bonjour"));
    // app.switch_project(SwitchProjectsDirection::Right);
    // app.add_task(String::from("Bonjour"));
    // app.switch_project(SwitchProjectsDirection::Left);
    // app.add_task(String::from("Coucou"));

    if let Err(e) = app.save_file() {
        eprintln!("{e}")
    };
    Ok(())
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.screen_mode {
                ScreenMode::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.save_file()?;
                        break;
                    }
                    KeyCode::Left => app.switch_project(SwitchProjectsDirection::Left),
                    KeyCode::Right => app.switch_project(SwitchProjectsDirection::Right),
                    KeyCode::Up => app.nav_tasks(TaskNavDirection::Up),
                    KeyCode::Down => app.nav_tasks(TaskNavDirection::Down),
                    KeyCode::Enter => app.toggle_task_state(),
                    KeyCode::Char('c') => app.cancel_task(),
                    KeyCode::Char('a') => app.screen_mode = ScreenMode::AddingTask,
                    _ => {}
                },
                ScreenMode::AddingTask => match key.code {
                    KeyCode::Esc => app.screen_mode = ScreenMode::Main,
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
