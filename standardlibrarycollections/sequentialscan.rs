fn main(){

    let vec = vec![10,20,30,40,50,60,];

    for item in &vec {
        println!("Item from Vec: {}", item);
    }

    use std::collections::VecDeque;
    let mut vec_dequeue = VecDeque::new();
    vec_dequeue.push_back(10);
    vec_dequeue.push_back(20);
    vec_dequeue.push_back(30);
    for item in &vec_dequeue {
        println!("Item from VecDeque: {}", item);
    }
}