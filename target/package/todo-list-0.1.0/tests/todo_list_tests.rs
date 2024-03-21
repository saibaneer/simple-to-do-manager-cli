use todo_list::{Task, TodoList};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_task() {
        let task = Task::new("Test Task".to_string());
        assert_eq!(task.task, "Test Task");
        assert!(!task.done);
    }

    #[test]
    fn mark_task_done() {
        let mut task = Task::new("Test Task".to_string());
        task.mark_done();
        assert!(task.done);
    }

    #[test]
    fn mark_task_undone() {
        let mut task = Task::new("Test Task".to_string());
        task.mark_done();
        task.mark_undone();
        assert!(!task.done);
    }

    #[test]
    fn edit_task_activity_name() {
        let mut task = Task::new("Old Task Name".to_string());
        task.edit_activity_name("New Task Name".to_string());
        assert_eq!(task.task, "New Task Name");
    }

    #[test]
    fn add_task_to_todo_list() {
        let mut list = TodoList::new();
        list.add_task("New Task".to_string());
        assert_eq!(list.tasks.len(), 4);
        assert_eq!(list.tasks[3].task, "New Task");
    }

    #[test]
    fn remove_task_from_todo_list() {
        let mut list = TodoList::new();
        list.add_task("Task to be removed".to_string());
        list.remove_task(3);
        assert_eq!(list.tasks.len(), 3);
    }

    #[test]
    fn mark_task_status_in_todo_list() {
        let mut list = TodoList::new();
        list.add_task("Task to be marked".to_string());
        list.mark_task_status(0, true);
        assert!(list.tasks[0].done);
    }

    #[test]
    fn edit_task_in_todo_list() {
        let mut list = TodoList::new();
        list.add_task("Old Task Name".to_string());
        list.edit_task(0, "New Task Name".to_string());
        assert_eq!(list.tasks[0].task, "New Task Name");
    }
}
