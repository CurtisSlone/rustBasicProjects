struct Mystruct{
    name: String,
}

impl Drop for Mystruct{
    fn drop(&mut self){
        println!("Dropping {}", self.name);
    }
}

fn main(){
    let my_struct = Mystruct{name: String::from("Hello")};
    // my_struct goes out of scope here and is dropped
}