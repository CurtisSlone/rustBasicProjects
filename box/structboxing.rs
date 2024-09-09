struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    let boxed_point = Box::new(point);

    println!("Point coordinates: ({}, {})", boxed_point.x, boxed_point.y);
}