fn main() {
    // 1. Creating a Vector
    let mut numbers: Vec<i32> = Vec::new(); // Empty vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("Numbers: {:?}", numbers);

    // 2. Accessing Elements
    if let Some(first) = numbers.get(0) {
        println!("The first element is: {}", first);
    } else {
        println!("The vector is empty.");
    }

    // 3. Iterating Over a Vector
    for num in &numbers {
        println!("Number: {}", num);
    }

    // 4. Modifying Elements
    for num in &mut numbers {
        *num *= 2; // Multiply each element by 2
    }
    println!("Doubled Numbers: {:?}", numbers);

    // 5. Removing Elements
    if let Some(removed) = numbers.pop() {
        println!("Popped element: {}", removed);
    }
    println!("After pop: {:?}", numbers);

    numbers.remove(0); // Remove by index
    println!("After remove: {:?}", numbers);

    // 6. Pattern Matching with Vectors
    match &numbers[..] {
        [first, second, ..] => println!("First: {}, Second: {}, Rest exist", first, second),
        [single] => println!("Only one element: {}", single),
        _ => println!("Different pattern."),
    }

    // 7. Using Iterators
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    // 8. Combining Generics and Vectors
    let generic_vector: Vec<Point<i32>> = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];

    for point in &generic_vector {
        println!("Point: ({}, {})", point.x, point.y);
    }
}

// 9. Generic Struct to Use with Vectors
struct Point<T> {
    x: T,
    y: T,
}
