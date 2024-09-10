fn first_n_elements(arr: &[i32], n: usize) -> &[i32] {
    &arr[..n]
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6,];
    let slice = first_n_elements(&arr, 3);
    println!("{:?}", slice);
}