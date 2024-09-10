fn split_string(s: &String, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
}

fn main() {
    let my_string = String::from("apple,banana,cherry");
    let split = split_string(&my_string, ',');
    for part in split {
        println!("{}", part);
    }
}