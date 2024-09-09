enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        }
    }
}

fn main(){
    let circle = Shape::Circle(5.0);
    println!("Circle area: {}", circle.area());

    let rectangle = Shape::Rectangle(3.0, 4.0);
    println!("Rectangle area: {}", rectangle.area());

    let square = Shape::Square(5.0);
    println!("Square area: {}", square.area());
}