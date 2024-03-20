// Add the `clap` crate as a dependency
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// `Task` represents a single task in the to-do list with its status.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    task: String,
    done: bool,
}

impl Task {
    /// Creates a new `Task` with the given activity name and default status as not done.
    fn new(activity: String) -> Task {
        Task {
            task: activity,
            done: false,
        }
    }

    /// Marks the task as done.
    fn mark_done(&mut self) {
        self.done = true;
    }

    /// Marks the task as not done.
    fn mark_undone(&mut self) {
        self.done = false;
    }

    /// Edits the activity name of the task.
    fn edit_activity_name(&mut self, new_name: String) {
        self.task = new_name;
    }
}

/// `TodoList` contains a list of tasks and provides functionality to manipulate them.
#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    /// Creates a new `TodoList`, attempting to load existing tasks from a file or initializing an empty list.
    fn new() -> TodoList {
        match TodoList::load("tasks.json") {
            Ok(todo_list) => todo_list,
            Err(_) => TodoList { tasks: Vec::new() },
        }
    }

    /// Loads the to-do list from the specified file.
    fn load(file_path: &str) -> Result<TodoList, std::io::Error> {
        let data = fs::read_to_string(file_path)?;
        let todo_list = serde_json::from_str(&data)?;
        Ok(todo_list)
    }

    /// Saves the to-do list to the specified file.
    fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string(&self)?;
        fs::write(file_path, serialized)?;
        Ok(())
    }

    /// Adds a new task with the given activity name to the to-do list.
    fn add_task(&mut self, activity: String) {
        let new_task = Task::new(activity);
        self.tasks.push(new_task);
    }

    /// Lists all tasks with their status.
    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "done" } else { "not done" };
            println!("{}: {} ({})", index, task.task, status);
        }
    }

    /// Gets and displays a task by its index in the list.
    fn get_task(&self, index: usize) {
        if let Some(task) = self.tasks.get(index) {
            let status = if task.done { "done" } else { "not done" };
            println!("Task {}: {} ({})", index, task.task, status);
        } else {
            println!("Invalid index");
        }
    }

    /// Removes a task by its index in the list.
    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task removed!");
        } else {
            println!("Invalid index");
        }
    }

    /// Marks a task's status as done or not done based on the given boolean value.
    fn mark_task_status(&mut self, index: usize, status: bool) {
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
    fn edit_task(&mut self, index: usize, new_name: String) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.edit_activity_name(new_name.clone());
            println!("Task {} name updated to '{}'", index, new_name);
        } else {
            println!("Invalid index");
        }
    }
}

// Entry point of the application
fn main() {
    // Initialize a new TodoList, either by loading from file or creating a new one
    let mut todo_list = TodoList::new();

    // Define the command line interface structure using `clap`
    let matches = Command::new("ToDoList")
        .version("1.0")
        .about("A simple to do list CLI")
        .author("Gbenga Ajiboye")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("task")
                     .help("The task to add")
                     .required(true)
                     .index(1),
                ),
        )
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(
            Command::new("get")
                .about("Get specific task by index")
                .arg(Arg::new("index")
                     .help("The index of the item you want to return")
                     .required(true)
                     .index(1),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a task by index")
                .arg(Arg::new("index")
                     .help("The index of the item you want to remove")
                     .required(true)
                     .index(1),
                ),
        )
        .subcommand(
            Command::new("mark_status")
                .about("Mark a task as done or not done")
                .arg(Arg::new("index")
                     .help("The index of the item you want to mark")
                     .required(true)
                     .index(1),
                )
                .arg(Arg::new("status")
                     .help("The status to set: true for done, false for not done")
                     .required(true)
                     .index(2),
                ),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a task by index")
                .arg(Arg::new("index")
                     .help("The index of the item you want to edit")
                     .required(true)
                     .index(1),
                )
                .arg(Arg::new("new_name")
                     .help("The new name of the task")
                     .required(true)
                     .index(2),
                ),
        )
        .get_matches();

    // Match the user's command and execute the corresponding functionality
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            todo_list.add_task(task.to_string());
            println!("Task added: {}", task);
            todo_list.save("tasks.json").unwrap(); // Save the updated list to file
        }
        Some(("list", _)) => {
            todo_list.list_tasks(); // Display all tasks
        }
        Some(("get", sub_m)) => {
            let index_str = sub_m.get_one::<String>("index").unwrap();
            let index = index_str.parse::<usize>().expect("Invalid index");
            todo_list.get_task(index); // Display the specified task
        }
        Some(("remove", sub_m)) => {
            let index_str = sub_m.get_one::<String>("index").unwrap();
            let index = index_str.parse::<usize>().expect("Invalid index");
            todo_list.remove_task(index); // Remove the specified task
            todo_list.save("tasks.json").unwrap(); // Save the updated list to file
        }
        Some(("mark_status", sub_m)) => {
            let index_str = sub_m.get_one::<String>("index").unwrap();
            let status_str = sub_m.get_one::<String>("status").unwrap();
            let index = index_str.parse::<usize>().expect("Invalid index");
            let status = status_str.parse::<bool>().expect("Invalid status");
            todo_list.mark_task_status(index, status); // Update the status of the specified task
            todo_list.save("tasks.json").unwrap(); // Save the updated list to file
        }
        Some(("edit", sub_m)) => {
            let index_str = sub_m.get_one::<String>("index").unwrap();
            let new_name = sub_m.get_one::<String>("new_name").unwrap();
            let index = index_str.parse::<usize>().expect("Invalid index");
            todo_list.edit_task(index, new_name.to_string()); // Edit the name of the specified task
            todo_list.save("tasks.json").unwrap(); // Save the updated list to file
        }
        _ => {
            println!("Invalid command provided!"); // Handle invalid commands
        }
    }
}
