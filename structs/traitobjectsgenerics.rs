trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
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

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn main() {
    let rectangle = Rectangle { width: 5.0, height: 10.0 };
    println!("Rectangle Area: {}", rectangle.area());
    println!("Rectangle Perimeter: {}", rectangle.perimeter());

    let circle = Circle { radius: 3.0 };
    println!("Circle Area: {}", circle.area());
    println!("Circle Perimeter: {}", circle.perimeter());
}