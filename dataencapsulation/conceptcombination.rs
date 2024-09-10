mod geometry {
    pub type Meters = u32;

    pub struct Rectangle {
        width: Meters,
        height: Meters,
    }

    impl Rectangle {
        pub fn new(width: Meters, height: Meters) -> Self {
            Rectangle { width, height }
        }

        pub fn area(&self) -> Meters {
            self.width * self.height
        }
    }
}

fn main() {
    let rectangle = geometry::Rectangle::new(3, 4);
    println!("Area: {}", rectangle.area());
}