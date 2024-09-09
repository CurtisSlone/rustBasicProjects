fn main(){
    let num = 5;
    let r1 = &num as *const i32;
    unsafe {
        println!("Value at r1: {}", *r1);
    }
}