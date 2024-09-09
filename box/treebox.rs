struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main(){
    let left_child = TreeNode { value: 2, left: None, right: None };
    let right_child = TreeNode { value: 3, left: None, right: None };

    let root = TreeNode {
        value: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };

    println!("Root value: {}", root.value);
    if let Some(left) = &root.left {
        println!("Left child value: {}", left.value);
    }

    if let Some(right) = &root.right {
        println!("Right child value: {}", right.value);
    }
}