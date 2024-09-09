fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let num1 = 5;
    let num2 = 10;
    println!("The largest number is: {}", largest(num1, num2));

    let str1 = "apple";
    let str2 = "banana";
    println!("The largest string is: {}", largest(str1, str2));
}