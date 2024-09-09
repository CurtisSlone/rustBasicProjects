enum BinaryTree {
    Empty,
    NonEmpty(Box<TreeNode>),
}

struct TreeNode {
    value: i32,
    left: BinaryTree,
    right: BinaryTree,
}

// In-Order Traversal
fn in_order(tree: &BinaryTree) {
    match tree {
        BinaryTree::Empty => (),
        BinaryTree::NonEmpty(node) => {
            in_order(&node.left);
            println!("{}", node.value);
            in_order(&node.right);
        }
    }
}

//Pre-Order traversal
fn pre_order(tree: &BinaryTree) {
    match tree {
        BinaryTree::Empty => (),
        BinaryTree::NonEmpty(node) => {
            println!("{}", node.value);
            pre_order(&node.left);
            pre_order(&node.right);
        }
    }
}

//Post-Order Traversal
fn post_order(tree: &BinaryTree) {
    match tree {
        BinaryTree:: Empty => (),
        BinaryTree::NonEmpty(node) => {
            post_order(&node.left);
            post_order(&node.right);
            println!("{}", node.value);
        }
    }
}

//Level-Order Traversal
fn level_order(tree: &BinaryTree) {

    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    if let BinaryTree::NonEmpty(node) = tree {
        queue.push_back(node);
    }

    while let Some(current) = queue.pop_front() {
        print!("{}", current.value);
        if let BinaryTree::NonEmpty(ref left) = current.left {
            queue.push_back(left);
        }
        if let BinaryTree::NonEmpty(ref right) = current.right {
            queue.push_back(right);
        }
    }
}

fn main() {
    use BinaryTree::*;

    let left_child = BinaryTree::NonEmpty(Box::new(TreeNode {
        value: 1,
        left: Empty,
        right: Empty,
    }));

    let right_child = BinaryTree::NonEmpty(Box::new(TreeNode {
        value: 3,
        left: Empty,
        right: Empty,
    }));

    let root = BinaryTree::NonEmpty(Box::new(TreeNode {
        value: 2,
        left: left_child,
        right: right_child,
    }));

    println!("In-Order Traversal:");
    in_order(&root);
    println!();

    println!("Pre-Order Traversal:");
    pre_order(&root);
    println!();

    println!("Post-Order Traversal:");
    post_order(&root);
    println!();

    println!("Level-Order Traversal:");
    level_order(&root);
    println!();
}