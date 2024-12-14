// Define a Classic Struct
struct Person {
    name: String,
    age: u8,
    address: Address, // Nested struct
}

// Define another Classic Struct for nesting
struct Address {
    city: String,
    zip_code: u32,
}

// Define a Tuple Struct
struct Point3D(f64, f64, f64);

// Define a Unit-Like Struct
struct Marker;

fn main() {
    // Creating an instance of the Address struct
    let home_address = Address {
        city: String::from("New York"),
        zip_code: 10001,
    };

    // Creating an instance of the Person struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        address: home_address,
    };

    // Accessing fields of the Person struct
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("City: {}", person.address.city);

    // Using the Tuple Struct
    let point = Point3D(1.0, 2.0, 3.0);
    println!("Point coordinates: ({}, {}, {})", point.0, point.1, point.2);

    // Using the Unit-Like Struct
    let _marker = Marker;
    println!("Unit-like struct instantiated.");

    // Demonstrating Struct Update Syntax
    let person2 = Person {
        name: String::from("Bob"),
        ..person
    };
    println!("New person: {} lives in {}.", person2.name, person2.address.city);

}

