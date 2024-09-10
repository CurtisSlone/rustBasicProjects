use std::fs::File;
use std::io::{Write, Read};

fn main(){
    let data = vec![1,2,3,4,5,6,7,8];

    let mut file = File::create("binary_data.bin").unwrap();
    file.write_all(&data).unwrap();

    let mut file = File::open("binary_data.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    println!("Read binary data {:?}", buffer);
}