extern "C" {
    fn sqrt(x: f64) -> f64;
}

fn main() {
    let x: f64 = 9.0;
    unsafe {
        println!("The square root of {} is {}", x, sqrt(x));
    }
}
