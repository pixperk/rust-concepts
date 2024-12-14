use std::f64::consts::PI;

// Define an Enum for simple states
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Define an Enum with data
enum Shape {
    Circle(f64),        // Holds the radius
    Rectangle(f64, f64), // Holds width and height
    Triangle { base: f64, height: f64 }, // Holds named fields
}

// Define an Enum with methods
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

fn main() {
    // Using the simple enum
    let light = TrafficLight::Red;
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Prepare to stop."),
        TrafficLight::Green => println!("Go!"),
    }

    // Using the enum with data
    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(5.0, 8.0);
    let triangle = Shape::Triangle { base: 6.0, height: 4.0 };

    println!("Area of the circle: {:.2}", circle.area());
    println!("Area of the rectangle: {:.2}", rectangle.area());
    println!("Area of the triangle: {:.2}", triangle.area());

}
