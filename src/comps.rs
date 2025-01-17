use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::{
    cmp::Ordering,
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    vec,
};

// CursorManager stuff

pub enum CursorDirection {
    Left,
    Right,
}

pub struct CursorManager {
    pub string: String,
    pub cursor_position: u16,
}

impl CursorManager {
    pub fn new() -> Self {
        CursorManager {
            string: String::new(),
            cursor_position: 0,
        }
    }

    pub fn insert(&mut self, c: char) {
        self.string.insert(self.cursor_position as usize, c);
        self.cursor_position += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor_position > 0 {
            self.string.remove(self.cursor_position as usize - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor(&mut self, dir: CursorDirection) {
        match dir {
            CursorDirection::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            CursorDirection::Right => {
                if self.cursor_position < self.string.len() as u16 {
                    self.cursor_position += 1;
                }
            }
        }
    }

    pub fn validate(&mut self) -> String {
        let s = self.string.clone();
        self.string.clear();
        self.cursor_position = 0;
        s
    }

    pub fn set_string(&mut self, s: String) {
        self.string = s;
        self.cursor_position = self.string.len() as u16;
    }

    pub fn clear(&mut self) {
        self.string.clear();
        self.cursor_position = 0;
    }
}

// File manager stuff
pub struct FileManager {
    file_path: PathBuf,
}

impl FileManager {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let mut file_manager = FileManager {
            file_path: PathBuf::new(),
        };
        file_manager.get_file_path()?;
        Ok(file_manager)
    }

    pub fn get_file_path(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file_path = PathBuf::from(env::var("HOME").unwrap());

        file_path.push(Path::new(".tuido"));
        if !file_path.exists() {
            fs::create_dir(file_path.clone())?;
        }

        file_path.push(Path::new("tasks.json"));
        if !file_path.exists() {
            fs::File::create(file_path.clone())?;
        }

        self.file_path.push(file_path);
        Ok(())
    }

    pub fn save_file(&mut self, projects: &Vec<Project>) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create(self.file_path.clone())?;
        to_writer_pretty(file, projects)?;
        Ok(())
    }

    pub fn open_file(&mut self) -> Result<Vec<Project>, Box<dyn Error>> {
        let file = fs::File::open(self.file_path.clone())?;
        let projects = from_reader(file);

        if projects.is_ok() {
            return Ok(projects.unwrap());
        } else {
            return Ok(vec![]);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskState {
    Todo,
    Completed,
    Canceled,
}

// Task stuff

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    project_id: u32,
    pub state: TaskState,
    pub title: String,
}

impl Task {
    pub fn new(id: u32, project_id: u32, title: String) -> Self {
        Task {
            id,
            project_id,
            state: TaskState::Todo,
            title,
        }
    }

    pub fn toggle_state(&mut self) {
        match self.state {
            TaskState::Todo => self.state = TaskState::Completed,
            TaskState::Canceled => self.state = TaskState::Todo,
            TaskState::Completed => self.state = TaskState::Todo,
        }
    }

    pub fn cancel(&mut self) {
        match self.state {
            TaskState::Canceled => {}
            _ => self.state = TaskState::Canceled,
        }
    }

    pub fn rename(&mut self, new_title: String) {
        self.title = new_title.clone();
    }
}

// Project stuff

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub is_current: bool,
    pub tasks: Vec<Task>,
}

impl Project {
    pub fn new(id: u32, name: String) -> Self {
        Project {
            id,
            name,
            is_current: false,
            tasks: vec![],
        }
    }

    pub fn toggle_current(&mut self) {
        self.is_current = !self.is_current;
    }
    pub fn add_task(&mut self, id: u32, title: String) {
        self.tasks.push(Task::new(id, self.id, title));
    }

    pub fn delete_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    pub fn toggle_task_state(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.toggle_state();
            }
        }
    }

    pub fn cancel_task(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.cancel();
            }
        }
    }

    pub fn rename_task(&mut self, id: u32, new_title: String) {
        for task in &mut self.tasks {
            if task.id == id {
                task.rename(new_title.clone());
            }
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name.clone();
    }
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Project {}

// Tests

#[cfg(test)]
mod file_manager_test {
    use super::*;

    #[test]
    fn build() {
        assert!(FileManager::build().is_ok())
    }
}

#[cfg(test)]
mod project_test {
    use super::*;

    #[test]
    fn add_task() {
        let mut project = Project::new(1, String::from("Projet 1"));

        project.add_task(1, String::from("Bonjour"));

        assert_eq!(project.tasks.len(), 1);
        assert_eq!(project.tasks[0].title, String::from("Bonjour"))
    }
    #[test]
    fn delete_task() {
        let mut project = Project::new(1, String::from("Projet 1"));

        project.add_task(1, String::from("Bonjour"));
        project.add_task(2, String::from("Salut"));

        project.delete_task(1);

        assert_eq!(project.tasks.len(), 1);
        assert_eq!(project.tasks[0].title, String::from("Salut"))
    }
}

#[cfg(test)]
mod task_test {
    use super::*;

    #[test]
    fn toggle_state() {
        let mut task = Task::new(1, 1, "Tache".to_string());

        assert!(matches!(task.state, TaskState::Todo));
        task.toggle_state();
        assert!(matches!(task.state, TaskState::Completed));
        task.toggle_state();
        assert!(matches!(task.state, TaskState::Todo));
    }

    #[test]
    fn cancel() {
        let mut task = Task::new(1, 1, "Tache".to_string());

        task.cancel();
        assert!(matches!(task.state, TaskState::Canceled));
        task.toggle_state();
        assert!(matches!(task.state, TaskState::Todo));
    }

    #[test]
    fn rename() {
        let mut task = Task::new(1, 1, "Salut".to_string());

        task.rename("Bonjour".to_string());
        assert_eq!(task.title, "Bonjour".to_string())
    }
}
