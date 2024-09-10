fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("The result is: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("The result is: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}