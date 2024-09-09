#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref DATA: Mutex<Vec<i32>> = Mutex::new(vec![1, 2, 3, 4, 5]);
}

fn add_value(val: i32){
    let mut data = DATA.lock().unwrap();
    data.push(val);
}

fn main() {
    add_value(6);
    println!("{:?}", DATA.lock().unwrap());
}