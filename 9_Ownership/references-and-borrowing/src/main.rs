fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);    // Pass a reference to s1
    
    println!("The length of '{}' is {}.", s1, len);
    
    let mut s2 = String::from("hello");
    change(&mut s2);                    // Pass a mutable reference
}

fn calculate_length(s: &String) -> usize {
    s.len()
}   // s goes out of scope, but because it's a reference,
    // nothing happens to the value it refers to

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}