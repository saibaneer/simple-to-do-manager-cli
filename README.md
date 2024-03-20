# ToDoList CLI Application

A simple command-line interface (CLI) application to manage a to-do list.

## Features

- Add new tasks
- List all tasks
- Get a specific task by index
- Remove a task by index
- Mark a task as done or not done
- Edit the name of a task

## Installation

Before you can use the ToDoList CLI, you need to ensure you have Rust installed on your machine. Follow the instructions on [the official Rust website](https://www.rust-lang.org/learn/get-started) to install Rust.

## Usage

After installing Rust, you can run the application using Cargo:

```bash```
cargo run

## Adding a Task
To add a new task to your to-do list, use the add subcommand:
`cargo run -- add "Your task here"`


## Listing All Tasks
To list all tasks in your to-do list, use the list subcommand:
`cargo run -- list`


## Getting a specific Task
To get a specific task by its index, use the get subcommand:
`cargo run -- get <index>`
Replace <index> with the numeric index of the task you want to view.

## Removing a Task
To remove a task from your to-do list by its index, use the remove subcommand:
`cargo run -- remove <index>`
Replace <index> with the numeric index of the task you want to remove.


## Marking a Task as Done or Not Done
To change the status of a task, use the mark_status subcommand:
`cargo run -- mark_status <index> <status>`
Replace <index> with the task's index and <status> with true to mark as done, or false to mark as not done.

## Editing a Task
To edit the name of a task, use the edit subcommand:
`cargo run -- edit <index> "New task name"`
Replace <index> with the task's index and "New task name" with the new name for the task.

## Author
-- Gbenga Ajiboye