fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = "Hello, world!";
    let s2 = "Rust is awesome!";
    println!("The longest string is: {}", longest(s1, s2));
}