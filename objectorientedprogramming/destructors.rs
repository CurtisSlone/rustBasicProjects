struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let r = Resource {name: String::from("Resource A")};
    println!("Resource created");
}