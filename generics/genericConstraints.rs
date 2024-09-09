fn print_item<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

fn main() {
    let int_value = 5;
    let float_value = 3.14;
    let string_value = "Hello, World!";

    print_item(int_value);
    print_item(float_value);
    print_item(string_value);
}