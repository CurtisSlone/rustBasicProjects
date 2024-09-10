use std::env;

fn main(){
    let key = "MY_VAR";

    match env::var(key) {
        Ok(value) => println!("{}={}", key, value),
        Err(_) => println!("{} not found", key),
    }

    env::set_var(key, "newValue");
    println!("{}={}", key, env::var(key).unwrap());
}
