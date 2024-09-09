fn print_string(s: &String){
    println!("{}", s);
}

fn append_woirld(s: &mut String){
    s.push_str(" world");
}

fn main(){
    let mut greeting = String::from("Hello");
    print_string(&greeting);
    append_woirld(&mut greeting);
    print_string(&greeting);  // Output: Hello world
}