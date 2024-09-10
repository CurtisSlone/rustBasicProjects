fn format_string(s: String, num: i32) -> String {
    format!("{}: {}", s, num)
}

fn main() {
    let fruit = String::from("apple");
    let result = format_string(fruit, 5);
    println!("{}", result);
}