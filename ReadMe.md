# TODO TUI app in Rust

## Pitch for the app

TuiDo helps you to organize tasks into projects, add information to tasks...

## Things to do

- [ ] Define app behavior
- [ ] App structure (see JSON-Editor tutorial)
- [ ] Define UI organization
- [ ] Define Project and Task structures 
- [ ] Organize tasks storage

## App behavior

You see the project you are in.
You can switch between projects with `TAB` and `SHIFT+TAB`.
You can navigate up and down trough tasks with `UP` and `DOWN` 
You can add, delete or rename a task with `a`, `d` and `r`.

You can switch to projects view with `p`.
In that view, you can navigate up and down trough projects with `UP` and `DOWN` 
You can add, delete or rename a project with `a`, `d` and `r`.


## App structure

```
TuiDo
|__ main.rs
|__ app.rs
    |__ App
        |__ projects 
        |__ file_manager
        |__ next_task_id
        |__ current_mode (E)
            |__ Normal
            |__ AddingTask
            |__ AddingProject
            |__ Edidting Task
|__ ui.rs
```

