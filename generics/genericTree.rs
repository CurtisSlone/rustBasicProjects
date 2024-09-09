use std::fmt::Display;

enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T: Ord + Clone + Display> BinaryTree<T> {
    // Insert a value into the binary tree
    fn insert(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Node {
                    value,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                };
            }
            BinaryTree::Node { ref mut left, ref mut right, ref value } => {
                if value > &value {
                    left.insert(value.clone());
                } else if value < &value {
                    right.insert(value.clone());
                }
            }
        }
    }

    // Find the minimum value in the binary tree
    fn find_min(&self) -> Option<T> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::Node { value, left, .. } => {
                if let BinaryTree::Empty = **left {
                    Some(value.clone())
                } else {
                    left.find_min()
                }
            }
        }
    }

    // Delete a value from the binary tree
    fn delete(self, value: T) -> BinaryTree<T> {
        match self {
            BinaryTree::Empty => BinaryTree::Empty,
            BinaryTree::Node { value: v, left, right } => {
                if value < v {
                    BinaryTree::Node {
                        value: v,
                        left: Box::new(left.delete(value)),
                        right,
                    }
                } else if value > v {
                    BinaryTree::Node {
                        value: v,
                        left,
                        right: Box::new(right.delete(value)),
                    }
                } else {
                    // Node with value `v` found
                    if let BinaryTree::Empty = *left {
                        *right
                    } else if let BinaryTree::Empty = *right {
                        *left
                    } else {
                        // Node with two children
                        let min = right.find_min().unwrap();
                        BinaryTree::Node {
                            value: min.clone(),
                            left,
                            right: Box::new(right.delete(min)),
                        }
                    }
                }
            }
        }
    }

    // Recursive traversal: in-order traversal
    fn inorder(&self) {
        match self {
            BinaryTree::Empty => {}
            BinaryTree::Node { value, left, right } => {
                left.inorder();
                print!("{} ", value);
                right.inorder();
            }
        }
    }
}

fn main() {
    // Create a new binary tree
    let mut tree = BinaryTree::Empty;

    // Insert values into the binary tree
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(2);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    println!("In-order traversal:");
    tree.inorder(); // Should print: 2 3 4 5 6 7 8

    // Delete a value from the binary tree
    tree = tree.delete(3);

    println!("\nIn-order traversal after deleting 3:");
    tree.inorder(); // Should print: 2 4 5 6 7 8
}
