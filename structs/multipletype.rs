struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

fn main() {
    let p: Point<f64, f32> = Point::new(3.0, 4.0);
    println!("Point coordinates: ({}, {})", p.x, p.y);
}