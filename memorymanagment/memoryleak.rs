use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
    prev: RefCell<Option<Weak<Node>>>,
}

fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
        prev: RefCell::new(None),
    });

    let node2 = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&node1))),
        prev: RefCell::new(None),
    });

    *node1.prev.borrow_mut() = Some(Rc::downgrade(&node2));

    // Memory leak is avoided due to weak references in node1.prev
    println!("Node1 strong count: {}", Rc::strong_count(&node1)); // 1
    println!("Node2 strong count: {}", Rc::strong_count(&node2)); // 1
}
