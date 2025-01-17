use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{
    app::{App, ScreenMode},
    comps::TaskState,
};

struct Stats {
    tasks: u32,
    completed: u32,
    canceled: u32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            tasks: 0,
            completed: 0,
            canceled: 0,
        }
    }

    pub fn add_task(&mut self, state: &TaskState) {
        self.tasks += 1;
        match state {
            TaskState::Completed => self.completed += 1,
            TaskState::Canceled => self.canceled += 1,
            _ => {}
        }
    }

    pub fn get_string(&self) -> String {
        format!(
            " Tasks: {} - Completed: {} - Canceled: {} ",
            self.tasks, self.completed, self.canceled
        )
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)])
        .split(f.area());

    // Affichage projet en cours

    //let project_block = Block::default()
    //    .borders(Borders::ALL)
    //    .style(Style::default());

    //let project = Paragraph::new(Text::styled(
    //    format!("< {} >", app.get_current_project_name()),
    //    Style::default().fg(Color::Green),
    //))
    //.alignment(Alignment::Center)
    //.block(project_block);

    //f.render_widget(project, chunks[0]);

    // Affichage des tâches

    let mut list_items = Vec::<ListItem>::new();
    let mut task_count: u16 = 0;

    let mut stats = Stats::new();

    if let Some(tasks) = app.get_current_project_tasks() {
        for task in tasks {
            task_count += 1;

            let fmt;
            let style;
            if task.id == app.current_task_id {
                match app.screen_mode {
                    ScreenMode::Main | ScreenMode::AddingTask | ScreenMode::RenamingProject => {
                        fmt = format!(" {} - {}", get_checkbox(&task.state), task.title);
                        style = get_style_selected(&task.state);
                    }
                    ScreenMode::DeletingTask => {
                        let text = format!("{}  [Delete Task? (y/n)]", task.title);
                        fmt = format!(" {} - {}", get_checkbox(&task.state), text);
                        style = Style::default().fg(Color::Black).bg(Color::Red);
                    }
                    ScreenMode::RenamingTask => {
                        fmt = format!(
                            " {} - {}",
                            get_checkbox(&task.state),
                            app.cursor_manager.string
                        );
                        style = Style::default().fg(Color::Yellow);
                        f.set_cursor_position(Position::new(
                            7 + app.cursor_manager.cursor_position,
                            task_count,
                        ));
                    }
                }
            } else {
                fmt = format!(" {} - {}", get_checkbox(&task.state), task.title);
                style = get_style(&task.state);
            }

            stats.add_task(&task.state);

            list_items.push(ListItem::new(Line::from(Span::styled(fmt, style))));
        }
    }

    // Ajout d'une tâche

    match app.screen_mode {
        ScreenMode::AddingTask => {
            task_count += 1;
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!(" 󰄰  - {}", app.cursor_manager.string),
                Style::default().fg(Color::Yellow),
            ))));
            f.set_cursor_position(Position::new(
                7 + app.cursor_manager.cursor_position,
                task_count,
            ));
        }
        ScreenMode::RenamingProject => {
            f.set_cursor_position(Position::new(4 + app.cursor_manager.cursor_position, 0));
        }
        _ => {}
    }

    let project_name;
    match app.screen_mode {
        ScreenMode::RenamingProject => project_name = app.cursor_manager.string.clone(),
        _ => project_name = app.get_current_project_name(),
    }

    let tasks_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(format!(" 🞀 {} 🞂 ", project_name))
        .title_bottom(stats.get_string());

    let list = List::new(list_items).block(tasks_block);

    f.render_widget(list, chunks[0]);

    // Affichage du footer

    //let stats_par = Paragraph::new(Line::from(stats.get_string()))
    //    .block(Block::default().borders(Borders::ALL));

    //let current_keys_hint = {
    //    match app.screen_mode {
    //        app::ScreenMode::Main => Span::styled(
    //            "(q) to quit / (a) to add task / (r) to rename task / (d) to delete task",
    //            Style::default().fg(Color::Red),
    //        ),
    //        app::ScreenMode::AddingTask | app::ScreenMode::RenamingTask => Span::styled(
    //            "(ESC) to quit / (ENTER) to validate",
    //            Style::default().fg(Color::Red),
    //        ),
    //    }
    //};

    //let key_notes_footer =
    //    Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    //let footer_chunks = Layout::default()
    //    .direction(Direction::Horizontal)
    //    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
    //    .split(chunks[1]);

    //f.render_widget(stats_par, footer_chunks[0]);
    //f.render_widget(key_notes_footer, footer_chunks[1]);
}

fn get_checkbox(state: &TaskState) -> String {
    match state {
        TaskState::Todo => "󰄰 ",
        TaskState::Canceled => "󰍶 ",
        TaskState::Completed => "󰗠 ",
    }
    .to_string()
}

fn get_style(state: &TaskState) -> Style {
    match state {
        TaskState::Todo => Style::default().fg(Color::White),
        TaskState::Canceled => Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::CROSSED_OUT),
        TaskState::Completed => Style::default().fg(Color::Green),
    }
}

fn get_style_selected(state: &TaskState) -> Style {
    match state {
        TaskState::Todo => Style::default().fg(Color::Black).bg(Color::LightBlue),
        TaskState::Canceled => Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::CROSSED_OUT)
            .bg(Color::LightBlue),
        TaskState::Completed => Style::default().fg(Color::Green).bg(Color::LightBlue),
    }
}
