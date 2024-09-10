fn main() {
    let stack_array: [i32; 1000] = [0; 1000];
    println!("Stack array size: {}", std::mem::size_of_val(&stack_array));
}