#[derive(Clone)]
struct LargeData {
    data: [i32; 1024],
}

fn main() {
    let large = LargeData { data: [0; 1024] };
    let boxed_large = Box::new(large.clone());
    println!("Stack-Allocation struct size: {}", std::mem::size_of_val(&large));
    println!("Heap-Allocation struct size: {}", std::mem::size_of_val(&boxed_large));
}