use comps::CursorDirection;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io::Stderr};

use app::{App, ScreenMode, SwitchProjectsDirection, TaskNavDirection};
use ui::ui;
use utils::{cleanup_terminal, init_terminal};

mod app;
mod comps;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = init_terminal::<CrosstermBackend<Stderr>>()?;
    let mut app = App::build()?;
    run_app(&mut terminal, &mut app)?;
    cleanup_terminal(&mut terminal)?;

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
                    KeyCode::Up | KeyCode::Char('k') => app.nav_tasks(TaskNavDirection::Up),
                    KeyCode::Down | KeyCode::Char('j') => app.nav_tasks(TaskNavDirection::Down),
                    KeyCode::Enter => app.toggle_task_state(),
                    KeyCode::Char('c') => app.cancel_task(),
                    KeyCode::Char('a') => app.screen_mode = ScreenMode::AddingTask,
                    KeyCode::Char('r') => {
                        app.screen_mode = ScreenMode::RenamingTask;
                        app.task_to_cursor_manager();
                    }
                    KeyCode::Char('d') => {
                        app.screen_mode = ScreenMode::DeletingTask;
                    }
                    KeyCode::Char('n') => {
                        app.add_project();
                        app.screen_mode = ScreenMode::RenamingProject;
                    }
                    KeyCode::Char('p') => {
                        app.screen_mode = ScreenMode::RenamingProject;
                        app.project_to_cursor_manager();
                    }
                    _ => {}
                },
                ScreenMode::AddingTask => match key.code {
                    KeyCode::Esc => {
                        app.screen_mode = ScreenMode::Main;
                        app.cursor_manager.clear();
                    }
                    KeyCode::Char(char) => app.cursor_manager.insert(char),
                    KeyCode::Backspace => app.cursor_manager.delete(),
                    KeyCode::Enter => {
                        let new_task = app.cursor_manager.validate();
                        app.add_task(new_task);
                        app.screen_mode = ScreenMode::Main;
                        app.save_file()?;
                    }
                    KeyCode::Right => app.cursor_manager.move_cursor(CursorDirection::Right),
                    KeyCode::Left => app.cursor_manager.move_cursor(CursorDirection::Left),
                    _ => {}
                },
                ScreenMode::RenamingTask => match key.code {
                    KeyCode::Esc => {
                        app.screen_mode = ScreenMode::Main;
                        app.cursor_manager.clear();
                    }
                    KeyCode::Char(char) => app.cursor_manager.insert(char),
                    KeyCode::Enter => {
                        let new_task_name = app.cursor_manager.validate();
                        app.rename_task(new_task_name);
                        app.screen_mode = ScreenMode::Main;
                        app.save_file()?;
                    }
                    KeyCode::Backspace => app.cursor_manager.delete(),
                    KeyCode::Right => app.cursor_manager.move_cursor(CursorDirection::Right),
                    KeyCode::Left => app.cursor_manager.move_cursor(CursorDirection::Left),
                    _ => {}
                },
                ScreenMode::RenamingProject => match key.code {
                    KeyCode::Esc => {
                        app.screen_mode = ScreenMode::Main;
                        app.cursor_manager.clear();
                    }
                    KeyCode::Char(char) => app.cursor_manager.insert(char),
                    KeyCode::Enter => {
                        let new_project_name = app.cursor_manager.validate();
                        app.rename_project(new_project_name);
                        app.screen_mode = ScreenMode::Main;
                        app.save_file()?;
                    }
                    KeyCode::Backspace => app.cursor_manager.delete(),
                    KeyCode::Right => app.cursor_manager.move_cursor(CursorDirection::Right),
                    KeyCode::Left => app.cursor_manager.move_cursor(CursorDirection::Left),
                    _ => {}
                },
                ScreenMode::DeletingTask => match key.code {
                    KeyCode::Esc | KeyCode::Char('n') => {
                        app.screen_mode = ScreenMode::Main;
                    }
                    KeyCode::Char('y') => {
                        app.delete_task(app.current_task_id);
                        app.save_file()?;
                        app.screen_mode = ScreenMode::Main;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
