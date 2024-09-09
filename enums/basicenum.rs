enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time_on(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 15,
        }
    }
}

fn main(){
    let light = TrafficLight::Green;
    println!("Traffic light is {} and it will be on for {} seconds", light, light.time_on());
}