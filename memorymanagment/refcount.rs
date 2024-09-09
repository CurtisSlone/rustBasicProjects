use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let node1 = Rc::new(Node { value: 1, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(Rc::clone(&node1)) });

    println!("node1 strong count = {}", Rc::strong_count(&node1)); // 2
    println!("node2 strong count = {}", Rc::strong_count(&node2)); // 1
}
