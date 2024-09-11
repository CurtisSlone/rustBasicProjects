use ::std::collections::VecDeque;

fn main(){

    let mut deque = VecDeque::new();

    deque.push_back(10);
    deque.push_back(20);

    let front = deque.pop_front();
    let back = deque.pop_back();

    println!("Front: {:?}", front);
    println!("Back: {:?}", back);
}