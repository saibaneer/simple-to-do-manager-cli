use serde::{Deserialize, Serialize};
use crate::Task;
use std::{fs, path::Path};


/// `TodoList` contains a list of tasks and provides functionality to manipulate them.
#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    /// Creates a new `TodoList`, attempting to load existing tasks from a file or initializing an empty list.
    pub fn new() -> TodoList {
        match TodoList::load("tasks.json") {
            Ok(todo_list) => todo_list,
            Err(_) => TodoList { tasks: Vec::new() },
        }
    }

    /// Loads the to-do list from the specified file.
    pub fn load(file_path: &str) -> Result<TodoList, std::io::Error> {
        let data = fs::read_to_string(file_path)?;
        let todo_list = serde_json::from_str(&data)?;
        Ok(todo_list)
    }

    /// Saves the to-do list to the specified file.
    pub fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string(&self)?;
        fs::write(file_path, serialized)?;
        Ok(())
    }

    /// Adds a new task with the given activity name to the to-do list.
    pub fn add_task(&mut self, activity: String) {
        let new_task = Task::new(activity);
        self.tasks.push(new_task);
    }

    /// Lists all tasks with their status.
    pub fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "done" } else { "not done" };
            println!("{}: {} ({})", index, task.task, status);
        }
    }

    /// Gets and displays a task by its index in the list.
    pub fn get_task(&self, index: usize) {
        if let Some(task) = self.tasks.get(index) {
            let status = if task.done { "done" } else { "not done" };
            println!("Task {}: {} ({})", index, task.task, status);
        } else {
            println!("Invalid index");
        }
    }

    /// Removes a task by its index in the list.
    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task removed!");
        } else {
            println!("Invalid index");
        }
    }

    /// Marks a task's status as done or not done based on the given boolean value.
    pub fn mark_task_status(&mut self, index: usize, status: bool) {
        if let Some(task) = self.tasks.get_mut(index) {
            if status {
                task.mark_done();
                println!("Task marked as done");
            } else {
                task.mark_undone();
                println!("Task marked as not done");
            }
        }
    }

    /// Edits the name of a task by its index in the list.
    pub fn edit_task(&mut self, index: usize, new_name: String) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.edit_activity_name(new_name.clone());
            println!("Task {} name updated to '{}'", index, new_name);
        } else {
            println!("Invalid index");
        }
    }
}
