fn main(){
    let v = vec![1, 2, 3, 4];

    let result: Vec<_> = v.iter().filter(|x| *x % 2 == 0).map(|x| *x * 2).collect();

    println!("Filtered and Mapped: {:?}", result);
}