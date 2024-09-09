use std::cell::RefCell;

struct MyData {
    value: RefCell<i32>,
}

fn main() {
    let data = MyData { value: RefCell::new(5) };
    let mut borrowed_data = data.value.borrow_mut();
    *borrowed_data += 10;
    println!("Modified value: {}", borrowed_data); // Prints: Modified value: 15
}