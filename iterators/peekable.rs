fn main(){
    let numbers = vec![1,2,3,4,5,6,7,];
    let mut iter = numbers.iter().peekable();

    while let Some(&num)  = iter.next() {
        if let Some(&next) = iter.peek() {
            println!("Curruent: {}, Next: {}", num, next);
        } else {   
            println!("Current: {}", num);
         }
    }

}