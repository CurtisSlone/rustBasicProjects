mod utilities {
    pub mod math {
        pub fn square(num: i32) -> i32 {
            num * num
        }
    }

    pub mod strings {
        pub fn greet(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    }
}

fn main() {
    let squared = utilities::math::square(5);
    let greeting  = utilities::strings::greet("Alice");

    println!("{}", greeting);
    println!("The square of 5 is: {}", squared);
}