use std::convert::TryFrom;

struct PositiveNumber {
    value: u32,
}

impl TryFrom<i32> for PositiveNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(PositiveNumber { value: value as u32 })
        } else {
            Err("Value must be positive")
        }
    }
}

fn main() {
    match PositiveNumber::try_from(0) {
        Ok(positive_number) => println!("Converted value: {}", positive_number.value),
        Err(error) => eprintln!("Error: {}", error),
    }
    match PositiveNumber::try_from(-5) {
        Ok(positive_number) => println!("Converted value: {}", positive_number.value),
        Err(error) => eprintln!("Error: {}", error),
    }
}