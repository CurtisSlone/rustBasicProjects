trait Drawable {
    type Shape;
    fn draw(&self, shape: &Self::Shape);
}

struct Circle;
struct Square;

struct Drawer;

impl Drawable for Drawer {
    type Shape = Circle;
    fn draw(&self, _shape: &Self::Shape) {
        println!("Drawing a circle");
    }
}

fn draw_shape<T>(drawer: &T, shape: &T::Shape)
where T: Drawable,
{
    drawer.draw(shape);
}


fn main() {
    let drawer = Drawer;
    let circle = Circle;
    draw_shape(&drawer, &circle);
}