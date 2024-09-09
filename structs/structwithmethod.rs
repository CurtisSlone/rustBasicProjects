struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rectangle = Rectangle { width: 3, height: 4 };
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());
}