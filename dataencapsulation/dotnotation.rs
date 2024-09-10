struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p = Point {x: 3, y: 4 };

    let dist1 = Point::distance_from_origin(&p);
    println!("Distance from origin: {}", dist1);

    let p2 = Point { x: -2, y: 0 };
    let dist2 = p2.distance_from_origin();
    println!("Distance (dot notation) from origin: {}", dist2);
}