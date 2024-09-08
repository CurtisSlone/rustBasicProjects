use std::io::{self, Write};

fn main() {
    loop{
        println!("Enter an expression or 'quit' to exit");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.to_lowercase() == "quit" {
            break;
        }

        match evaluate_expression(input) {
            Ok(x) => println!("Result: {}", x),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
   let parts: Vec<&str> = expression.split_whitespace().collect();

   if parts.len() != 3{
    return Err("Invalid Expression".to_string());
   }

   let num1: f64 = parts[0].parse().map_err((|_| "Invalid Expression".to_string()))?;
   let operator = parts[1];
   let num2: f64 = parts[2].parse().map_err((|_| "Invalid Expression".to_string()))?;

   match operator {
    "+" => Ok(num1 + num2),
    "-" => Ok(num1 - num2),
    "*" => Ok(num1 * num2),
    "/" => {
        if num2 == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(num1 / num2)
        }
    },
    _ => Err("Invalid operator".to_string()),
   }
    
}
