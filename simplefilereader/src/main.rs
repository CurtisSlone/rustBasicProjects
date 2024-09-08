use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    let path = &args[1];

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                line_count += 1;
                word_count += line.split_whitespace().count();
                char_count += line.chars().count();
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            }
        }
    }

    println!("Number of lines: {}", line_count);
    println!("Number of words: {}", word_count);
    println!("Number of characters: {}", char_count);
}
