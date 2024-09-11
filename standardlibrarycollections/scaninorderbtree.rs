fn main() {
    use std::collections::BTreeSet;
    
    let mut set = BTreeSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(5);

    for item in &set {
        println!("Item in sorted order: {}", item);
    }
}
