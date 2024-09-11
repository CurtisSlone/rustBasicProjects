use std::collections::BTreeSet;

fn main(){
    let mut set = BTreeSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    if let Some(&largest) = set.iter().next_back() {
        set.remove(&largest);
        println!("Largest number removed: {}", largest);
    }
}