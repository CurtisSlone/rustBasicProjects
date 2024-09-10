use std::ops::Add;

fn add_numbers<T>(a: T, b: T) -> T
where T: Add<Output = T> + Copy,
{
    a + b
}

fn main() {
    let sum = add_numbers(5, 10);
    println!("Sum: {}", sum);
}