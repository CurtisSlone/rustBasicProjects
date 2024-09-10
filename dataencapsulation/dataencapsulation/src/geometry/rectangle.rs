pub type Meters = u32;

pub struct Rectangle {
    pub width: Meters,
    pub height: Meters,
}

impl Rectangle {
    pub fn new(width: Meters, height: Meters) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> Meters {
        self.width * self.height
    }
}