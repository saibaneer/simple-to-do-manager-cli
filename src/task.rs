use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task: String,
    pub done: bool,
}

/// `Task` represents a single task in the to-do list with its status.


impl Task {
    /// Creates a new `Task` with the given activity name and default status as not done.
    pub fn new(activity: String) -> Task {
        Task {
            task: activity,
            done: false,
        }
    }

    /// Marks the task as done.
    pub fn mark_done(&mut self) {
        self.done = true;
    }

    /// Marks the task as not done.
    pub fn mark_undone(&mut self) {
        self.done = false;
    }

    /// Edits the activity name of the task.
    pub fn edit_activity_name(&mut self, new_name: String) {
        self.task = new_name;
    }
}