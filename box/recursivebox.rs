#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil
}

fn main(){
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Node(3, Box::new(List::Nil))))));
    println!("{:?}", list);
}