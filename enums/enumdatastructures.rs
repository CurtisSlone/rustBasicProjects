enum List {
    Cons(i32,Box<List>),
    Nil,
}

impl List {
    fn length(&self) -> u32 {
        match self {
            List::Cons(head, tail) => 1 + tail.length(),
            List::Nil => 0,
        }
    }
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("Length of the list: {}", list.length());
}