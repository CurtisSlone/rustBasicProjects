#[derive(Debug)]
enum MyError {
    ParseError(String),
    IOError(String),
}

type MyResult<T> = Result<T, MyError>;

fn parse_integer(input: &str) -> MyResult<i32> {
match input.parse::<i32>() {
    Ok(n) => Ok(n),
    Err(e) => Err(MyError::ParseError(format!("Failed to parse integer: {}", e))),
}
}

fn read_file(path: &str) -> MyResult<String> {
    if path == "valid_path.txt" {
        Ok(String::from("This is a valid file"))
    } else {
        Err(MyError::IOError(format!("Failed to read file at path: {}", path)))
    }
}

fn process_data(input: &str) -> MyResult<i32> {
    let parsed_data = parse_integer(input)?;
    let read_data = read_file("invalid_path.txt")?;
    Ok(parsed_data + read_data.len() as i32)
}

fn main() {
    match process_data("123") {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}