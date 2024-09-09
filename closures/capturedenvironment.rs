fn main(){
    let mut counter = 0;

    let mut increment_counter = || {
        counter += 1;
        counter
    };

    println!("Increment: {}", increment_counter());
    println!("Increment: {}", increment_counter());
}