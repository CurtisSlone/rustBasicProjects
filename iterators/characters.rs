fn main() {
    let s = String::from("hello");

    for ch in s.chars() {
        println!("{}", ch);
    }

    for byte in s.bytes() {
        println!("{}", byte);
    }
}