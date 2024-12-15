// Define a trait
trait Animal {
    fn make_sound(&self) -> String;
}

// Implement for different types
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

fn main() {
    // Create a vector of trait objects
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    // Use dynamic dispatch to call methods
    for animal in animals {
        println!("The animal says: {}", animal.make_sound());
    }
    
    // Using &dyn directly
    let dog = Dog;
    let cat = Cat;
    print_animal_sound(&dog);
    print_animal_sound(&cat);
}

// Function taking a trait object reference
fn print_animal_sound(animal: &dyn Animal) {
    println!("Sound: {}", animal.make_sound());
}