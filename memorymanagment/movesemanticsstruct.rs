struct Data {
    values: Vec<i32>,
}

fn take_ownership_of_data(data: Data) {
    println!("Data: {:?}", data.values);
}

fn main() {
    let data = Data {values: vec![1,2,3]};
    take_ownership_of_data(data);
}