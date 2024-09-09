struct CustomData;

impl Drop for CustomData {
    fn drop(&mut self) {
        println!("CustomData dropped");
    }
}

fn main() {
    let custom_data = CustomData;
    // custom_data goes out of scope here and is dropped
}