fn main(){
    let x = 5;
    let boxed_x = Box::new(x);

    println!("Original value: {}", x);
    println!("Value inside the box: {}", *boxed_x);
}