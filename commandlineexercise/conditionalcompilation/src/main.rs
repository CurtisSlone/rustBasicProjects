// conditional.rs
fn main() {
    #[cfg(feature = "fast")]
    println!("Fast version");

    #[cfg(not(feature = "fast"))]
    println!("Default version");
}
