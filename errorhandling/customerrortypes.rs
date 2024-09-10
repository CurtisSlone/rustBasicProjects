use std::fmt;

#[derive(Debug)]
enum ParseError {
    EmptyString,
    InvalidNumber
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyString => write!(f, "The input string is empty"),
            ParseError::InvalidNumber => write!(f, "Invalid number format")
        }
    }
}

fn parse_number(s: &str) -> Result<i32, ParseError> {
    if s.is_empty() {
        Err(ParseError::EmptyString)
    } else {
        s.parse::<i32>().map_err(|_| ParseError::InvalidNumber)
    }
}

fn main() {
    match parse_number("123") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => println!("Error: {}", error)
    }

    match parse_number("") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => println!("Error: {}", error)
    }

    match parse_number("abc") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => println!("Error: {}", error)
    }
}