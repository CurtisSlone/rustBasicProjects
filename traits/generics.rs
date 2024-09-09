fn show_info<T: std::fmt::Display + std::fmt::Debug>(item: T) {
    println!("{}", item);
    println!("{:?}", item);
}

fn main() {
    show_info(5);
    show_info("Hello, World!");
}