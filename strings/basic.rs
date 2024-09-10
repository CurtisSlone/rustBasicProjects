fn main() {
    let mut str1 = String::from("Hello");
    let str2 = String::from(" World");

    str1.push_str(&str2);
    println!("{}", str1); // Outputs: Hello World
}