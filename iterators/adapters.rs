fn main(){
    let v = vec![1, 2, 3, 4, 5];

    let even: Vec<_> = v.iter().filter(|x| *x %2 == 0).collect();

    println!("Even numbers: {:?}", even);

    let doubled: Vec<_> = v.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    for( idx, val) in v.iter().enumerate() {
        println!("Index: {:?}, Value: {:?}", idx, val);
    }
}