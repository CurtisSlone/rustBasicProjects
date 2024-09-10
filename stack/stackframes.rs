fn main() {
    let result = add(5,3);
    println!("The sum is: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum
}