#[derive(Copy, Clone)]
struct MyPoint {
    x: i32,
    y: i32,
}

fn main(){
    let p1 = MyPoint { x: 0, y: 0 };
    let p2 = p1; //Copy occurs
    println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y);
  }