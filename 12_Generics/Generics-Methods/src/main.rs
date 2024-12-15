struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // Constructor method
     fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}


fn main(){
    let Random_pt = Point::new(10, 14);
}
