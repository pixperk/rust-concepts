#[derive(Clone, Copy)] //derive copy and clone trait

struct Point{
    x:i32,
    y:i32
}


fn main() {
    let p1 = Point{x: 10, y:20};

    let p2 = p1;

    println!("p1: x:{}, y:{}", p1.x, p1.y); //No error(Copy instead of Move semantics)
    println!("p1: x:{}, y:{}", p2.x, p2.y);
}
