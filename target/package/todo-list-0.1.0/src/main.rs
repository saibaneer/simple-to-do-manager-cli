// Add the `clap` crate as a dependency
use clap::{Arg, Command};
use todo_list::TodoList;




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
