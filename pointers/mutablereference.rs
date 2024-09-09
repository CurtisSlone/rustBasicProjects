fn increment(x: &mut i32){
    *x +=1;
}

fn main(){
    let mut num = 5;
    increment(&mut num);
    println!("{}", num); // Output: 6
}