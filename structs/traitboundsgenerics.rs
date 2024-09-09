use std::ops::{Add, Sub};

struct Point<T: Add + Sub + Copy> {
    x: T,
    y: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy> Point<T> {
    fn distance(&self, other: &Point<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx + dy
    }
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 6, y: 8 };

    let distance = p1.distance(&p2);
    println!("Distance: {}", distance); // Output: 7
}