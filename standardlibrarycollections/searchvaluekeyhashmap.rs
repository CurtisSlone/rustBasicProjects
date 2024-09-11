use std::collections::HashMap;

fn main(){
    let mut map = HashMap::new();
    map.insert("Alice",30);
    map.insert("Bob",40);

    if let Some(age) = map.get("Alice") {
        println!("Alice's age: {}", age);
    } else {
        println!("Alice not found");
    }
}