fn take_ownership(s: String) {
    println!("String inside function: {}",s);
}

fn main() {
    let s = String::from("Hello, world!");
    take_ownership(s);
    // Uncommenting the line below will cause a compile-time error because `s` has been moved
    // println!("Original string: {}", s);
}