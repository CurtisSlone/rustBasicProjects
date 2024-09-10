use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("Error opening file: {}", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).with_context(|| format!("Error reading file: {}", path))?;
    Ok(contents)
}

fn main() -> Result<()> {
    let contents = read_file("example.txt")?;
    println!("File contents:\n{}", contents);
    Ok(())
}