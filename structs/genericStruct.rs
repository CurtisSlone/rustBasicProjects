struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

fn main() {
    let p1 = Point::new(3, 4);
    println!("Point: ({}, {})", p1.x, p1.y);

    let p2 = Point::new(5.5, 6.7);
    println!("Point: ({}, {})", p2.x, p2.y);

    let p3 = Point::new("Hello", "World");
    println!("Point: ({}, {})", p3.x, p3.y);
}
