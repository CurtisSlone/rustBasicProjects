use std::collections::{VecDeque, LinkedList};

fn main(){
    let mut deque = VecDeque::new();
    let mut linked_list = LinkedList::new();

    deque.push_front(20);
    deque.push_back(20);

    linked_list.push_back(20);
    linked_list.push_front(20);

    println!("Deque: {:?}", deque);
    println!("Linked List: {:?}", linked_list);
}