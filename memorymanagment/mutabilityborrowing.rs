fn append_text(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hello");
    append_text(&mut s);
    println!("{}", s); // Prints "Hello, world!"
}
