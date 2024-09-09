enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let ok_result: MyResult<i32, &str> = MyResult::Ok(42);
    let err_result: MyResult<i32, &str> = MyResult::Err("An error occurred");

    match ok_result {
        MyResult::Ok(val) => println!("Ok result: {}", val),
        MyResult::Err(err) => println!("Error: {}", err),
    }

    match err_result {
        MyResult::Ok(val) => println!("Ok result: {}", val),
        MyResult::Err(err) => println!("Error: {}", err),
    }
}