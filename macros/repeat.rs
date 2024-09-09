macro_rules! repeat_n {
    ($n:expr, $block:block) => {
        for _ in 0..$n {
            $block
        }
    };
}

fn main() {
    repeat_n!(5, {
        println!("Hello, world!");
    });
}