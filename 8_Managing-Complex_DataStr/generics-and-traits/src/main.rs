struct Point <T> {
    x: T,
    y: T,
}

trait Drawable{
    fn draw(&self);
}

struct Circle{
    radius : f64
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}
impl  Drawable for Point<i32> {
    fn draw(&self) {
        println!("Drawing lines with dimension x {} and y {}", self.x, self.y);
    }
}


fn main() {
    let int_pt = Point{x : 3, y : 4};
    int_pt.draw();
    let float_pt = Point{x : 3.5, y : 4.7};
    let c = Circle{radius : 4.0};
    c.draw();
}
