#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point1 = Point { x: 0, y: 0 };
    let point2 = point1.clone();

    println!("Point 1: {:?}", point1);
    println!("Point 2: {:?}", point2);

    println!("Are points equal? {}", point1 == point2);
}