use std::fs;
use std::io::{self, Write};
use std::path::Path;

const FILE_NAME: &str = "todo.txt";

fn main() {
    loop {
        println!("Choose an action:");
        println!("1. List tasks");
        println!("2. Add task");
        println!("3. Remove task");
        println!("4. Quit");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read input");
        let action: u32 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match action {
            1 => list_tasks(),
            2 => add_task(),
            3 => remove_task(),
            4 => break,
            _ => println!("Invalid action. Please choose a number between 1 and 4."),
        }
    }
}

fn list_tasks(){
    let tasks = read_tasks();
    if tasks.is_empty(){
        println!("No tasks found.");
    } else {
        println!("Tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{} - {}", index + 1, task);
        }
    }
}

fn add_task(){
    print!("Enter the task description: ");
    io::stdout().flush().expect("Failed to flush");

    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read input");
    let task = task.trim();

    if !task.is_empty() {
        let mut tasks = read_tasks();
        tasks.push(task.to_string());
        write_tasks(&tasks);
        println!("Task added successfully.");
    } else {
        println!("Task description cannot be empty");
    }
}

fn remove_task(){
    let tasks = read_tasks();
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    println!("Tasks:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{} - {}", index + 1, task);
    }

    print!("Enter the task number to remove: ");
    io::stdout().flush().expect("Failed to flush");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice. Please enter a valid number.");
            return;
        }
    };

    if choice > 0 && choice <= tasks.len() {
        let mut tasks = tasks;
        tasks.remove(choice - 1);
        write_tasks(&tasks);
        println!("Task removed successfully.");
    } else {
        println!("Invalid choice. Please enter a number between 1 and {}", tasks.len());
    }
}

fn read_tasks() -> Vec<String> {
    if Path::new(FILE_NAME).exists() {
        fs::read_to_string(FILE_NAME)
            .unwrap_or_default()
            .lines()
            .map(|line| line.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

fn write_tasks(tasks: &[String]) {
    let content = tasks.join("\n");
    fs::write(FILE_NAME, content).expect("Failed to write to file");
}