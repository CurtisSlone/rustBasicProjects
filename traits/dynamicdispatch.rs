trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 5.0, height: 3.0 }),
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 10.0, height: 6.0 }),
        Box::new(Circle { radius: 3.5 }),
    ];

    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}