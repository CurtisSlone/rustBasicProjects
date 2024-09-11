fn main(){
    let s1 = String::from("Hello, world!");
    let s2 = s1;
    // println!("{}", s1); // Output: error: borrow of moved value: `s1`
    println!("{}", s2); // Output: Hello, world!
}