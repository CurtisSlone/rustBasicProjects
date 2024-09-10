mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    // Cannot access add() because it is private
    // let sum = math::add(2, 3);  // This will cause an error

    let product = math::multiply(2, 3);
    println!("Product: {}", product);
}