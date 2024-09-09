use std::fmt;

macro_rules! impl_display {
    ($name: ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Hello, {}!", self.name)
            }
        }
    };
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl_display!(Person);

fn main() {
    let person = Person { name: "John".to_string(), age: 30 };
    println!("{}", person);
}