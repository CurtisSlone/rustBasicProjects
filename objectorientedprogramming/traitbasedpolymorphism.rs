trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with dimensions {}x{}", self.width, self.height);
    }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 10.0, height: 6.0 }),
    ];

    for shape in shapes {
        shape.draw();
    }
}