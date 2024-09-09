macro_rules! define_struct {
    ($name:ident { $($field:ident: $type:ty), *}) => {
        struct $name {
            $($field: $type), *
        }
    };
}

define_struct! { Point { x: i32, y: i32 } }

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point coordinates: ({}, {})", p.x, p.y);
}
