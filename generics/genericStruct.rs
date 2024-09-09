struct Point<T> {
    x: T,
    y: T,
}

fn main(){
    let p: Point<f64> = Point { x: 3.0, y: 4.0 };
    println!("Point coordinates: ({}, {})", p.x, p.y); // Output: Point coordinates: (3.0, 4.0)

    let p2: Point<i32> = Point { x: 5, y: 6 };
    println!("Point coordinates: ({}, {})", p2.x, p2.y); // Output: Point coordinates: (5, 6)
}