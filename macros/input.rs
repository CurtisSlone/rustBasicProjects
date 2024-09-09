macro_rules! print_value {
    ($val:expr) => {
        println!("Value: {} = {}",stringify!($val), $val);
    }
}

fn main() {
    print_value!(5);
    print_value!(3.14);
    print_value!("Hello, World!");// Error: std::env::args().next() returns an Option<String>, not a valid expression for print_value! macro.
}