use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    let path = "test.txt";

    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(content) => println!("{}", content),
                Err(_) => println!("Error reading line"),
            }
        }
    } else {
        println!("Could not openfile");
    }
}