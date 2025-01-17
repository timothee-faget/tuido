use std::{error::Error, vec};

use crate::comps::{CursorManager, FileManager, Project, Task};

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
    RenamingTask,
    RenamingProject,
    DeletingTask,
}

pub struct App {
    pub projects: Vec<Project>,
    pub file_manager: FileManager,
    pub next_task_id: u32,
    pub current_project_id: u32,
    pub current_task_id: u32,
    pub screen_mode: ScreenMode,
    pub cursor_manager: CursorManager,
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
            cursor_manager: CursorManager::new(),
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
            self.projects.push(Project::new(1, String::from("Welcome")));
            self.projects[0].add_task(1, String::from("Welcome in tuido"));
            self.projects[0].add_task(2, String::from("You'll love tuido"));
            self.projects[0].toggle_current();
            self.projects
                .push(Project::new(2, String::from("Other Project")));
            self.projects[1].add_task(2, String::from("Ypi can have mutiple projects"));
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
                    if *min_id >= self.current_task_id {
                        self.current_task_id = ids.iter().min().unwrap().clone();
                    }
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
                self.current_task_id = self.next_task_id;
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

        self.nav_tasks(TaskNavDirection::Up);
    }

    pub fn toggle_task_state(&mut self) {
        for project in &mut self.projects {
            if project.is_current {
                project.toggle_task_state(self.current_task_id);
                break;
            }
        }
    }

    pub fn rename_task(&mut self, new_title: String) {
        for project in &mut self.projects {
            if project.is_current {
                project.rename_task(self.current_task_id, new_title.clone());
                break;
            }
        }
    }

    pub fn rename_project(&mut self, new_name: String) {
        for project in &mut self.projects {
            if project.is_current {
                project.rename(new_name.clone());
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

    pub fn task_to_cursor_manager(&mut self) {
        for project in &mut self.projects {
            if project.is_current {
                let current_task = project
                    .tasks
                    .iter()
                    .find(|task| task.id == self.current_task_id)
                    .unwrap();
                self.cursor_manager.set_string(current_task.title.clone());
                break;
            }
        }
    }

    pub fn add_project(&mut self) {
        let id = self.projects.iter().map(|p| p.id).max().unwrap() + 1;
        self.projects
            .push(Project::new(id, String::from("New project")));
        self.projects
            .iter_mut()
            .find(|p| p.id == self.current_project_id)
            .unwrap()
            .toggle_current();
        self.projects.last_mut().unwrap().toggle_current();
        self.current_project_id = id;
    }

    pub fn project_to_cursor_manager(&mut self) {
        for project in &mut self.projects {
            if project.is_current {
                self.cursor_manager.set_string(project.name.clone());
                break;
            }
        }
    }
}
