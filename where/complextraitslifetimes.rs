fn find_larger<'a, T>(a: &'a T, b: &'a T) -> &'a T
where T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let x = 10;
    let y = 20;

    let larger = find_larger(&x, &y);
    println!("The larger number is: {}", larger);
}