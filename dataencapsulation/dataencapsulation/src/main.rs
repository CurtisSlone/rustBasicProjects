mod geometry;

fn main() {
    let rectangle = geometry::rectangle::Rectangle::new(3, 4);
    println!("Area: {}", rectangle.area());

    let circle = geometry::circle::Circle::new(5);
    println!("Area: {}", circle.area());
}