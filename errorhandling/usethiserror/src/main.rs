use thiserror::Error;
use std::io::{self, Read};

#[derive(Error,Debug)]
enum CustomError {
    #[error("I/O error occured: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),
}

fn read_file(path: &str) -> Result<String, CustomError> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn validate_input(input: &str) -> Result<String, CustomError> {
    if input.is_empty() {
        Err(CustomError::InvalidInput("Input cannot be empty".to_string()))
    } else {
        Ok(input.to_string())
    }
}

fn main() {
    match read_file("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }

    match validate_input("Hello, World!") {
        Ok(input) => println!("Valid input: {}", input),
        Err(error) => eprintln!("Error validating input: {}", error),
    }
}
