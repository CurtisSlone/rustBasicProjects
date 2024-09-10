fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let my_str = "Hello, World!";
    let reversed_strings = reverse_string(my_str);
    println!("{}", reversed_strings); // Output: !dlroW ,olleH
}