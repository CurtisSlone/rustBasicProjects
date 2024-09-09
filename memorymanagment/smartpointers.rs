struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    println!("x is of type: {}", std::any::type_name::<i32>());
    println!("y is of type: {}", std::any::type_name::<MyBox<i32>>());
}