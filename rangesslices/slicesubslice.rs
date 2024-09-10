fn find_subslice(haystack: &[i32], needle: &[i32]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn main() {
    let haystack = [1,2,3,4,5,6,7,8,];
    let needle = [4,5,6];
    if let Some(index) = find_subslice(&haystack, &needle) {
        println!("Found needle at index {}", index);
    } else {
        println!("Needle at index {:?}", haystack);
    }
}