use command_pattern::{CommandInvoker, AddTextCommand, RemoveTextCommand, Document};

fn main() {
    // Create a document
    let mut document = Document::new();

    // Create a command invoker
    let mut invoker = CommandInvoker::new();

    // Add text to the document
    {
        let mut add_command = AddTextCommand::new("Hello, world!");
        invoker.execute_command(Box::new(add_command));
        document.add_text("Hello, world!");
    }

    println!("Document after adding text: {}", document.text);

    // Remove text from the document
    {
        let mut remove_command = RemoveTextCommand::new(6);
        invoker.execute_command(Box::new(remove_command));
        document.remove_text(6);
    }

    println!("Document after removing text: {}", document.text);

    // Undo last command (removal)
    invoker.undo_last_command();
    println!("Document after undo: {}", document.text);

    // Undo another command (adding text)
    invoker.undo_last_command();
    println!("Document after undoing add: {}", document.text);
}
