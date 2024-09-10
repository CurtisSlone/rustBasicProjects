fn extract_substr(s: &str) -> &str {
    let bytes = s.as_bytes();
    let start = 1;
    let end = 5;
    &s[start..end]
}

fn main() {
    let my_str = "Hello, World!";
    println!("Substring: {}", extract_substr(my_str));
}