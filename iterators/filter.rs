fn main(){
    let number = vec![1,2,3,4,5,6,7];
    let evens: Vec<_> = number.iter().filter(|x| *x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
}