#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialEq> PartialEq for Pair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

fn main() {
    let p1 = Pair { first: 1, second: 2 };
    let p2 = Pair { first: 1, second: 2 };
    let p3 = Pair { first: 3, second: 4 };

    println!("p1 == p2: {}", p1 == p2); // Output: true
    println!("p1 == p3: {}", p1 == p3); // Output: false
}