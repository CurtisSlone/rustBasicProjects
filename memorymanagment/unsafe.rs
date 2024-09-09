fn main(){
    let x = Box::new(42);

    let ptr = Box::into_raw(x);

    unsafe {
        println!("Value at memory address: {}", *ptr);
        Box::from_raw(ptr);
    }
}