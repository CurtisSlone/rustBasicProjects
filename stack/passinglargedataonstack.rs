struct LargeData {
    data: [u8; 1024],
}

fn process_data(data: LargeData){
    println!("Processing");
}

fn main() {
    let large_data = LargeData { data: [0; 1024] };
    process_data(large_data);
}