fn main(){
    let add_five = |x: i32| x + 5;

    let result1 = add_five(10);
    let result2 = add_five(20);

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}