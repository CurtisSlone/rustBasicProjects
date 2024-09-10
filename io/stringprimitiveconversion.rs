fn main(){
    let num_str = "123";
    let invalid_str = "abc";

    match num_str.parse::<i32>() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(_) => println!("Invalid input"),
    }

    match invalid_str.parse::<i32>() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(_) => println!("Invalid input"),
    }
}