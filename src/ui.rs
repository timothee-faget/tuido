use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{self, App, ScreenMode, TaskState};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    let project_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        format!("< {} >", app.get_current_project_name()),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Center)
    .block(project_block);

    f.render_widget(title, chunks[0]);

    let tasks_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut list_items = Vec::<ListItem>::new();
    let mut task_count: u16 = 0;

    if let Some(tasks) = app.get_current_project_tasks() {
        for task in tasks {
            task_count += 1;
            let fmt;
            match task.state {
                TaskState::Todo => fmt = format!("[ ] - {}", task.title),
                TaskState::Canceled => fmt = format!("[-] - {}", task.title),
                TaskState::Completed => fmt = format!("[x] - {}", task.title),
            }
            let style;
            if task.id == app.current_task_id {
                style = Style::default().fg(Color::Black).bg(Color::LightGreen);
            } else {
                style = Style::default().fg(Color::LightGreen);
            }
            list_items.push(ListItem::new(Line::from(Span::styled(fmt, style))));
        }
    }

    let list = List::new(list_items).block(tasks_block);

    f.render_widget(list, chunks[1]);
    match app.screen_mode {
        ScreenMode::Main => {}
        #[allow(clippy::cast_possible_truncation)]
        ScreenMode::AddingTask => f.set_cursor_position(Position::new(7, 4 + task_count)),
    }

    let mode_footer = Paragraph::new(Line::from("Stats about projects"))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.screen_mode {
            app::ScreenMode::Main => Span::styled(
                "(q) to quit / (a) to add task",
                Style::default().fg(Color::Red),
            ),
            app::ScreenMode::AddingTask => {
                Span::styled("(ESC) to quit", Style::default().fg(Color::Red))
            }
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}
