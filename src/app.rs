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

// App stuff

pub enum SwitchProjectsDirection {
    Left,
    Right,
}

pub enum TaskNavDirection {
    Up,
    Down,
}

pub enum ScreenMode {
    Main,
    AddingTask,
}

pub struct App {
    pub projects: Vec<Project>,
    pub file_manager: FileManager,
    pub next_task_id: u32,
    pub current_project_id: u32,
    pub current_task_id: u32,
    pub screen_mode: ScreenMode,
}

impl App {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let mut app = App {
            projects: vec![],
            file_manager: FileManager::build().unwrap(),
            next_task_id: 0,
            current_project_id: 0,
            current_task_id: 0,
            screen_mode: ScreenMode::Main,
        };
        app.read_file()?;
        app.init_next_task_id();
        app.get_current_project_id();
        app.init_current_task_id();
        Ok(app)
    }

    pub fn read_file(&mut self) -> Result<(), Box<dyn Error>> {
        self.projects = self.file_manager.open_file()?;
        if self.projects.len() == 0 {
            // TODO: clean that
            self.projects.push(Project::new(1, String::from("Test 1")));
            self.projects[0].add_task(1, String::from("Welcome in tuido"));
            self.projects[0].add_task(4, String::from("You'll love tuido"));
            self.projects[0].toggle_current();
            self.projects.push(Project::new(2, String::from("Test 2")));
            self.projects[1].add_task(2, String::from("Bonjour"));
            self.projects.push(Project::new(3, String::from("Test 3")));
        }
        self.save_file()?;
        Ok(())
    }

    pub fn save_file(&mut self) -> Result<(), Box<dyn Error>> {
        self.file_manager.save_file(&self.projects)?;
        Ok(())
    }

    pub fn init_next_task_id(&mut self) {
        if let Some(next_task_id) = self
            .projects
            .iter()
            .flat_map(|proj| proj.tasks.iter())
            .map(|task| task.id)
            .max()
        {
            self.next_task_id = next_task_id + 1;
        } else {
            self.next_task_id = 1;
        }
    }

    pub fn get_current_project_id(&mut self) {
        for project in &self.projects {
            if project.is_current {
                self.current_project_id = project.id;
                break;
            }
        }
    }

    pub fn init_current_task_id(&mut self) {
        let ids: Vec<u32> = self
            .projects
            .iter()
            .find(|proj| proj.is_current)
            .unwrap()
            .tasks
            .iter()
            .map(|t| t.id)
            .collect();
        if let Some(id) = ids.iter().min() {
            self.current_task_id = id.clone();
        } else {
            self.current_task_id = 0;
        }
    }

    pub fn nav_tasks(&mut self, dir: TaskNavDirection) {
        let ids: Vec<u32> = self
            .projects
            .iter()
            .find(|proj| proj.is_current)
            .unwrap()
            .tasks
            .iter()
            .map(|t| t.id)
            .collect();
        match dir {
            TaskNavDirection::Up => match ids.iter().min() {
                Some(min_id) => {
                    if *min_id != self.current_task_id {
                        self.current_task_id = ids
                            .iter()
                            .filter(|&&id| id < self.current_task_id)
                            .max()
                            .unwrap()
                            .clone();
                    }
                }
                None => {}
            },
            TaskNavDirection::Down => match ids.iter().max() {
                Some(max_id) => {
                    if *max_id != self.current_task_id {
                        self.current_task_id = ids
                            .iter()
                            .filter(|&&id| id > self.current_task_id)
                            .min()
                            .unwrap()
                            .clone();
                    }
                }
                None => {}
            },
        }
    }

    pub fn switch_project(&mut self, dir: SwitchProjectsDirection) {
        let mut ids = vec![];
        for project in &mut self.projects {
            ids.push(project.id);
            if project.is_current {
                project.toggle_current();
            }
        }
        ids.sort();

        let next_id;

        match dir {
            SwitchProjectsDirection::Right => {
                if self.current_project_id == ids.iter().max().unwrap().clone() {
                    next_id = ids.iter().min().unwrap().clone();
                    //eprintln!("Next id (reset Right) :  {next_id}");
                } else {
                    next_id = ids
                        .iter()
                        .filter(|&&id| id > self.current_project_id)
                        .min()
                        .unwrap()
                        .clone();
                    //eprintln!("Next id (normal Right) : {next_id}");
                }
            }
            SwitchProjectsDirection::Left => {
                if self.current_project_id == ids.iter().min().unwrap().clone() {
                    next_id = ids.iter().max().unwrap().clone();
                } else {
                    next_id = ids
                        .iter()
                        .filter(|&&id| id < self.current_project_id)
                        .max()
                        .unwrap()
                        .clone()
                }
            }
        }

        for project in &mut self.projects {
            if project.id == next_id {
                project.toggle_current();
                self.current_project_id = next_id;
            }
        }
        self.init_current_task_id();
    }

    pub fn get_current_project_name(&self) -> String {
        self.projects
            .iter()
            .find(|prj| prj.is_current)
            .unwrap()
            .name
            .clone()
    }

    pub fn get_current_project_tasks(&self) -> Option<&Vec<Task>> {
        self.projects
            .iter()
            .find(|proj| proj.id == self.current_project_id)
            .map(|prj| &prj.tasks)
    }

    pub fn add_task(&mut self, title: String) {
        for project in &mut self.projects {
            if project.is_current {
                project.add_task(self.next_task_id, title.clone());
                self.next_task_id += 1;
                break;
            }
        }
    }

    pub fn delete_task(&mut self, id: u32) {
        for project in &mut self.projects {
            if project.is_current {
                project.delete_task(id);
                break;
            }
        }
    }

    pub fn toggle_task_state(&mut self) {
        for project in &mut self.projects {
            if project.is_current {
                project.toggle_task_state(self.current_task_id);
                break;
            }
        }
    }

    pub fn cancel_task(&mut self) {
        for project in &mut self.projects {
            if project.is_current {
                project.cancel_task(self.current_task_id);
                break;
            }
        }
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
    id: u32,
    name: String,
    is_current: bool,
    tasks: Vec<Task>,
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
    //pub fn get_tasks(&self) -> Vec<&Task> {
    //    let a: Vec<&Task> = self.tasks.as_mut();
    //}
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
