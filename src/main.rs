mod models;
mod task_manager;
mod xml_parser;

use crate::models::Task;
use std::error::Error;
use std::io::stdin;
use std::{env, io};
use task_manager::TaskManager;
use xml_parser::{read, write};

static mut TASK_MANAGER: TaskManager = TaskManager { tasks: Vec::new() };

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_command = &args[1];

    if user_command == "new" {
        let new_task = create_new_task().unwrap();
        dbg!(new_task);
    }
}

fn create_new_task() -> Result<Task, Box<dyn Error>> {
    println!("Enter task description: ");
    let mut task_description = String::new();
    stdin().read_line(&mut task_description).unwrap();

    println!("Enter task due date: ");
    let mut task_due_date = String::new();
    stdin().read_line(&mut task_due_date).unwrap();

    println!("Is this task important? (y/n): ");
    let mut important_task = String::new();
    stdin().read_line(&mut important_task).unwrap();

    Ok(Task {
        description: task_description.to_string(),
        due_date: task_due_date.to_string(),
        important: important_task.to_string(),
    })
}
