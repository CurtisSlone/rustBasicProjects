struct Color(u8, u8, u8);

fn to_grayscale(color: Color) -> u8 {
    ((color.0 as u16 + color.1 as u16 + color.2 as u16) / 3) as u8
}

fn main() {
    let red = Color(255, 0, 0);
    let grayscale_red = to_grayscale(red);
    println!("Grayscale value of red: {}", grayscale_red);
}