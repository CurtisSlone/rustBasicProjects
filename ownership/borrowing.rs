fn main(){
    let s = String::from("Hello, world!");
    let s2 = &s;
    println!("{}", s2); // Output: Hello, world!
}