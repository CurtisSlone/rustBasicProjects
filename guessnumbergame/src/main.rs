use rand::Rng;
use std::io::{self, Write};

fn main() {
    let min = 1;
    let max = 100;

    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("Guess the number between {} and {}", min, max);

    loop {
        print!("Please enter your guess");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if guess < min || guess > max {
            println!("Number out of range. Please guess between {} and {}", min, max);
            continue;
        } else if guess < secret_number {
            println!("Too low! Try again.");
        } else if guess > secret_number {
            println!("Too high! Try again.");
        } else {
            println!("Congratulations! You guessed the number correctly!");
            break;
        }
    }
}
