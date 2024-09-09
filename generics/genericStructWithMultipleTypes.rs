struct Point<T, U> {
    x: T,
    y: U,
}

// Function to calculate the distance between two points
fn distance(p1: &Point<f64, f64>, p2: &Point<f64, f64>) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 }; // Use f64
    let p2 = Point { x: 3.0, y: 4.0 }; // Use f64

    println!("Distance: {}", distance(&p1, &p2));
}
