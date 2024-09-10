fn main(){
    let numbers = vec![1,2,3,4,5,6,7,8,9];

    let result: Vec<_> = numbers.iter().filter(|x| *x %2 != 0).map(|&x| x * x).collect();

    println!("Odd squares: {:?}", result);
}