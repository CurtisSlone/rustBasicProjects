macro_rules! match_enum {
    ($value:expr, $( $pattern:pat => $result:expr ), * ) => {
        match $value {
            $($pattern => $result), *
        }
    };
}

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    use Color::*;
    let color = Red;
    println!("{}", match_enum!(color, Red => "Red", Green => "Green", Blue => "Blue")); // Output: Red
}