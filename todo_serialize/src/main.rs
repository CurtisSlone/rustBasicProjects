use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, title: &str) {
        self.todos.push(Todo {
            title: title.to_string(),
            completed: false,
        });
    }

    fn list(&self) {
        for (index, todo) in self.todos.iter().enumerate() {
            println!("{}: [{}] {}", index + 1, if todo.completed { "X" } else { " " }, todo.title);
        }
    }

    fn save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let json = fs::read_to_string(filename)?;
        let todos = serde_json::from_str(&json)?;
        Ok(todos)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut todo_list = TodoList::new();
    let filename = "todos.json";

    // Load existing todos from file
    if let Ok(loaded_list) = TodoList::load(filename) {
        todo_list = loaded_list;
    }

    loop {
        println!("\nTodo List:");
        todo_list.list();

        println!("\nOptions:");
        println!("1. Add Todo");
        println!("2. Save and Exit");
        println!("3. Exit without Saving");

        print!("Choose an option: ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                print!("Enter todo title: ");
                io::stdout().flush()?;
                let mut title = String::new();
                io::stdin().read_line(&mut title)?;
                todo_list.add(title.trim());
            }
            2 => {
                todo_list.save(filename)?;
                println!("Todos saved to {}", filename);
                break;
            }
            3 => break,
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}
