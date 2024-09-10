fn main(){
    let mut v = vec![1,2,3,4,5];

    for item in v.iter_mut() {
        *item += 1;
    }

    println!("{:?}", v);
}