use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    value: i32,
    neighbors: Vec<Weak<RefCell<Node>>>,

}

fn main() {
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        neighbors: vec![],
    }));

    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        neighbors: vec![],
    }));

    let node3 = Rc::new(RefCell::new(Node {
        value: 3,
        neighbors: vec![],
    }));

    node1.borrow_mut().neighbors.push(Rc::downgrade(&node2));
    node2.borrow_mut().neighbors.push(Rc::downgrade(&node3));
    node3.borrow_mut().neighbors.push(Rc::downgrade(&node2));

    println!("{:?}", node1);
    println!("{:?}", node2);
    println!("{:?}", node3);
}