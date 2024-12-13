fn main() {
    let arr = [1,2,3,4,5];
    let tup = ("takuma", true,2);
    let slice = &arr[1..3];
    println!("Slice, {:?}", slice);

    
    let s = "Hello &str";
    println!("s :{s}");

    let str = String::from(s);
    println!("str :{str}");


    let mut string: String = String::from("Hello String");
    string.push_str("world");
    println!("string :{string}");

    


}
