use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let first = Rc::new(RefCell::new(Node {value: 0, next: None}));
    let second = Rc::new(RefCell::new(Node {value:1, next: None}));
    first.borrow_mut().next = Some(Rc::clone(&second));
}