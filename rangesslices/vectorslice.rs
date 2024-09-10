fn slice_vec(vec: &Vec<i32>, start: usize, end: usize) -> &[i32] {
    &vec[start..end]
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let slice = slice_vec(&vec, 2, 5);
    println!("{:?}", slice);
}