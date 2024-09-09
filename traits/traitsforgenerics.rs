struct Pair<T> {
    first: T,
    second: T,
}

trait Compare {
    fn compare(&self) -> &str;
}

impl<T: PartialOrd> Compare for Pair<T> {
    fn compare(&self) -> &str {
        if self.first > self.second {
            "First is greater"
        } else if self.first < self.second {
            "Second is greater"
        } else {
            "Both are equal"
        }
    }
}

fn main() {
    let pair = Pair { first: 5, second: 3 };
    println!("{}", pair.compare());

    let pair_floats = Pair { first: 2.2, second: 3.3 };
    println!("{}", pair_floats.compare());
}