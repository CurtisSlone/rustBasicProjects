use std::collections::{HashMap, BTreeMap};

fn main() {
    let mut hash_map = HashMap::new();
    let mut btree_map = BTreeMap::new();

    for i in 0..1000 {
        hash_map.insert(i, i);
        btree_map.insert(i, i);
    }

    let hash_search = hash_map.get(&500);
    println!("HashMap search result: {:?}", hash_search);

    let btree_search = btree_map.get(&500);
    println!("BTreeMap search result: {:?}", btree_search);
}
