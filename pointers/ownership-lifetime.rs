fn largest<'a>(numbers: &'a[i32]) -> &'a i32 {
    let mut max = &numbers[0];
    for number in numbers {
        if number > max {
            max = number;
        }
    }
    max
}

fn main() {
    let nums = vec![10, 20, 30, 40];
    let result = largest(&nums);
    println!("The largest number is: {}", result); // Output: The largest number is: 40
}