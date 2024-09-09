macro_rules! sum_all {
    ($($x:expr), *) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    }
}

fn main() {
    println!("{}", sum_all!(1, 2, 3, 4, 5)); 
}