fn replace_substring(s: &mut String, old: &str, new: &str) {
    *s = s.replace(old, new);
}

fn main() {
    let mut my_string = String::from("Hello, old World!");
    replace_substring(&mut my_string, "old", "new");
    println!("{}", my_string); // Output: Hello, new World!
}