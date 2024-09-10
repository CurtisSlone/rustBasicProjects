use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..101);
    println!("Random number between 1 and 100: {}", random_number);
}