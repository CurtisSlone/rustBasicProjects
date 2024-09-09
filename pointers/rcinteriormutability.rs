use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    value: i32,
    neighbors: Vec<NodeRef>,
}

fn main() {
    let node1 = Rc::new(RefCell::new(Node { value: 1, neighbors: vec![] }));
    let node2 = Rc::new(RefCell::new(Node { value: 2, neighbors: vec![] }));
    let node3 = Rc::new(RefCell::new(Node { value: 3, neighbors: vec![] }));

    node1.borrow_mut().neighbors.push(Rc::clone(&node2));
    node2.borrow_mut().neighbors.push(Rc::clone(&node3));
    node3.borrow_mut().neighbors.push(Rc::clone(&node1));

    //causes reference cycle
    // may cause memory leaks
}