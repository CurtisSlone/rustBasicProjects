use std::rc::Rc;

enum Tree {
    Node(i32, Rc<Tree>, Rc<Tree>),
    Nil,
}

fn main() {
    let leaf = Rc::new(Tree::Node(3, Rc::new(Tree::Nil), Rc::new(Tree::Nil)));
    let branch = Rc::new(Tree::Node(2, Rc::clone(&leaf), Rc::clone(&leaf)));
}