use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter something: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    println!("You entered: {}", input.trim());
}