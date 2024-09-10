fn main(){
    let integer = 42;
    let float = 3.14;
    let boolean = true;

    let int_str = integer.to_string();
    let float_str = float.to_string();
    let boolean_str = boolean.to_string();

    println!("Integer: {}", int_str);
    println!("Float: {}", float_str);
    println!("Boolean: {}", boolean_str);
}