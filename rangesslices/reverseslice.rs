fn reverse_slice(slice: &[i32]) -> Vec<i32> {
    let mut reversed = slice.to_vec();
    reversed.reverse();
    reversed
}

fn main(){
    let slice = [0,1,2,3,4,5,6];
    let reversed_slice = reverse_slice(&slice);
    println!("Reversed slice: {:?}", reversed_slice);
}