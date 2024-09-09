fn main(){
    let s = String::from("hello world");
    let consume = |s: String| {
        println!("{:?}", s);
    };

    consume(s);
    // Uncommenting the line below will cause a compile-time error because `s` has been moved
    // println!("Original string: {}", s); 
}