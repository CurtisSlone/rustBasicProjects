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

fn operate_generic<T: Shape>(shape: &T){
    println!("Area: {}", shape.area());
}

fn operate_dynamic(shape: &dyn Shape){
    println!("Area (dynamic): {}", shape.area());
}

fn main() {
    let rectangle = Rectangle { width: 5.0, height: 3.0 };
    let circle = Circle { radius: 2.0 };

    operate_generic(&rectangle);
    operate_generic(&circle);

    operate_dynamic(&rectangle);
    operate_dynamic(&circle);
}