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

fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    let rectangle = Rectangle { width: 5.0, height: 3.0 };
    let circle = Circle { radius: 2.0 };

    print_area(&rectangle);
    print_area(&circle);
}