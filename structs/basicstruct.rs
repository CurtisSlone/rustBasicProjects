struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let point1 = Point { x: 0, y: 0 };
    let point2 = Point { x: 3, y: 4 };
    let distance = point1.distance(&point2);
    println!("The distance between ({}, {}) and ({}, {}) is: {:.2}", point1.x, point1.y, point2.x, point2.y, distance);
}