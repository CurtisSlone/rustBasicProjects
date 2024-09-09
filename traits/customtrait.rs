struct Person {
    name: String,
    age: u32,
}

trait Greet {
    fn greet(&self);
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

fn main() {
    let person = Person { name: "John Doe".to_string(), age: 30 };
    person.greet();
}