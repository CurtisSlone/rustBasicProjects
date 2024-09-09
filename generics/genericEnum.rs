enum MyOption<T> {
    Some(T),
    None,
}

fn main(){
    let x: MyOption<i32> = MyOption::Some(5);
    let y: MyOption<i32> = MyOption::None;

    match x {
        MyOption::Some(n) => println!("The value is {}", n),
        MyOption::None => println!("The value is None"),
    }

    match y {
        MyOption::Some(n) => println!("The value is {}", n),
        MyOption::None => println!("The value is None"),
    }
}