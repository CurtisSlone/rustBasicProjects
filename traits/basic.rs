struct Person {
    name: String,
    age: u32,
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hello, {}!", self.name)
    }
}

fn main() {
    let person = Person { name: "John Doe".to_string(), age: 30 };
    println!("{:?}", person); //Debug
    println!("{}", person); //Display
}