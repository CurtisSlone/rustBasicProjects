fn recursive_function(n: u32){
    if n == 0 {
        return;
    }

    println!("{}", n);
    recursive_function(n - 1);
}

fn main() {
    recursive_function(10000);
}