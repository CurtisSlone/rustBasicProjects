static NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];

fn sum_array() -> i32 {
    let mut sum = 0;
    for &num in NUMBERS.iter() {
        sum += num;
    }
    sum
}

fn main() {
    println!("Sum of the array elements: {}", sum_array());
}