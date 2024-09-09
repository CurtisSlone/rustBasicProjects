use std::fmt::Display;

fn main(){
    let print_item = |item: &dyn Display| println!("{}", item);

    print_item(&42);
    print_item(&"Hello, World!");
}