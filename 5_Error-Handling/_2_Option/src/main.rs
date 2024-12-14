fn is_even(num : i32)->Option<bool>{
    if num%2 == 0 {
     Some(true)
    }else if num % 2 == 1 {
     Some(false)
    }else{
    None
    }
}


fn main() {
    let numbers :   Vec<i32> = vec![];
    let first = numbers.first();
    match first {
        Some(val)=>println!("First Elem : {val}"),
        None => println!("Empty vector")
    }

    let num = -1;
    match is_even(num) {
        Some(true) => println!("It is odd"),
        Some(false) => println!("It is even"),
        None => println!("It is negative")
    }
}
