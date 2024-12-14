struct User {
    name: String,
    age: u8,
    email: String,
}



fn main() {
    let numbers: Vec<i32> = vec![];
    match &numbers[..]{
       [first, rest@..] => println!("First : {}, Rest : {:?}", first, rest),
       _ => println!("Empty vector")
    }

    let user = User {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    match user {
        User { name, age: 18..=30, email } => {
            println!("User is in the young adult range: {} ({})", name, email);
        }
        User { age, .. } if age > 30 => {
            println!("User is older than 30.");
        }
        _ => println!("User does not match any criteria."),
    }

}
