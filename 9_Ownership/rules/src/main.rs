fn main() {
    let s1 = String::from("Hello World");
    println!("{s1}");

    let s2 = s1; //Ownership moved to s2
    //println!("{s1}"); throws error as it is a moved value
    println!("{s2}");

    {
        let s = String::from("Str");
        println!("{s}");
    }
    //Value of s is dropped after the scope
    
}
