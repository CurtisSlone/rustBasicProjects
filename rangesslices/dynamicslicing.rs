fn modify_slice<F>(slice: &mut [i32], f: F) ->  Vec<i32>
where F: Fn(i32) -> i32 {
    slice.iter_mut().map(|x| f(*x)).collect()
}

fn main() {
    let mut slice = [1, 2, 3, 4, 5];
    let modified = modify_slice(&mut slice, |x| x * 2);
    println!("Modified slice: {:?}", modified);
}