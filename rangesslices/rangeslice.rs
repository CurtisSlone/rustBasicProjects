fn slice_in_range(slice: &[i32], range: std::ops::Range<usize>) -> &[i32] {
    &slice[range]
}

fn main() {
    let slice = [1, 2, 3, 4, 5, 6];
    let result = slice_in_range(&slice, 2..5);
    println!("{:?}", result); // Output: [3, 4, 5]
}

