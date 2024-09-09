macro_rules! create_enum {
    ($name:ident { $($variant:ident),* }) => {
        enum $name {
            $($variant),*
        }
    };
}

create_enum!(Color {Red, Green, Blue});

fn main() {
    let color = Color::Green;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}