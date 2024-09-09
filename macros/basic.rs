macro_rules! hello_macro {
    () => {
        println!("Hello, world!");
    }
}

fn main() {
    hello_macro!();
}