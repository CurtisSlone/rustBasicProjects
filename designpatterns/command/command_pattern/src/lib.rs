// Trait for the Command pattern
pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

// Receiver: Document that stores the text
pub struct Document {
    pub text: String,
}

impl Document {
    pub fn new() -> Self {
        Document { text: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn remove_text(&mut self, length: usize) {
        let len = self.text.len();
        if len >= length {
            self.text.truncate(len - length);
        }
    }
}

// Concrete Command for adding text to the document
pub struct AddTextCommand {
    text: String,
    length: usize,
}

impl AddTextCommand {
    pub fn new(text: &str) -> Self {
        AddTextCommand {
            text: text.to_string(),
            length: text.len(),
        }
    }
}

impl Command for AddTextCommand {
    fn execute(&mut self) {
        println!("Executing AddTextCommand: adding '{}'", self.text);
    }

    fn undo(&mut self) {
        println!("Undoing AddTextCommand: removing '{}' characters", self.length);
    }
}

// Concrete Command for removing text from the document
pub struct RemoveTextCommand {
    length: usize,
}

impl RemoveTextCommand {
    pub fn new(length: usize) -> Self {
        RemoveTextCommand { length }
    }
}

impl Command for RemoveTextCommand {
    fn execute(&mut self) {
        println!("Executing RemoveTextCommand: removing '{}' characters", self.length);
    }

    fn undo(&mut self) {
        println!("Undoing RemoveTextCommand: cannot undo yet.");
    }
}

// Invoker: Stores commands and executes them
pub struct CommandInvoker {
    history: Vec<Box<dyn Command>>,
}

impl CommandInvoker {
    pub fn new() -> Self {
        CommandInvoker {
            history: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, mut command: Box<dyn Command>) {
        command.execute();
        self.history.push(command);
    }

    pub fn undo_last_command(&mut self) {
        if let Some(mut command) = self.history.pop() {
            command.undo();
        } else {
            println!("No commands to undo.");
        }
    }
}
