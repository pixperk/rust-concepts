fn generic_fn<T>(param : T)->T{
    param
}

enum generic_enum<T>{
    Some(T),
    None
}

struct generic_struct<T>{
    field : T
}

impl <T> generic_struct<T>{
    fn get_field(&self)->&T{
        &self.field
    }
}

fn largest<T: PartialOrd>(list : &[T])->&T{
    let mut largest = &list[0];
    for num in list.iter(){
        if num > largest{
            largest = num;
        }
    }
    largest
}

fn main() {
    // Using the generic function
    let value = generic_fn(42);
    println!("Value from generic function: {}", value);

    let text = generic_fn("Hello, Rust!".to_string());
    println!("Value from generic function: {}", text);

    // Using the generic enum
    let some_value: generic_enum<i32> = generic_enum::Some(100);
    let none_value: generic_enum<i32> = generic_enum::None;

    match some_value {
        generic_enum::Some(val) => println!("Some value: {}", val),
        generic_enum::None => println!("No value"),
    }

    match none_value {
        generic_enum::Some(val) => println!("Some value: {}", val),
        generic_enum::None => println!("No value"),
    }

    // Using the generic struct
    let generic_instance = generic_struct { field:87 };
    println!("Field value from generic struct: {}", generic_instance.get_field());

    let arr = [3,65,3,534,5,-3];
    let largest_num = largest(&arr);

    println!("largest : {largest_num}");


}
