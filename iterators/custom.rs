struct Fibonacci {
    current: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}

fn main() {
    let fib = Fibonacci::new();
    let first_ten: Vec<u32> = fib.take(10).collect();
    println!("First 10 numbers in the Fibonacci sequence: {:?}", first_ten);
}