trait Summable {
    fn my_sum(a: Self, b: Self) -> Self;
}

impl Summable for i32 {
    fn my_sum(a: Self, b: Self) -> Self {
        a + b
    }
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    println!("Sum: {}", i32::my_sum(x,y));
}