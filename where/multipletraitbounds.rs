use std::ops::Add;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Add<Output = T> + Copy + From<i32> + std::ops::Div<Output = T>,
{
    fn midpoint(&self, other: &Point<T>) -> Point<T> {
        Point {
            x: (self.x + other.x) / (T::from(2)),
            y: (self.y + other.y) / (T::from(2)),
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 30, y: 40 };

    let mid = p1.midpoint(&p2);
    println!("Midpoint: ({}, {})", mid.x, mid.y);
}
