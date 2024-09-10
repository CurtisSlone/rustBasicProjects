pub type Meters = u32;

pub struct Circle {
    pub radius: u32,
}

impl Circle {
    pub fn new(radius: u32) -> Self {
        Circle { radius }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64).powi(2)
    }
}