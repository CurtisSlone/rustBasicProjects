use std::fs::File;
use std::io::{self, Read, Write};
use std::error::Error;

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("example.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}