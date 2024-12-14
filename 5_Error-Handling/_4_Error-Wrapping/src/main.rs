use std::fmt;
use std::fs::File;
use std::io::{self, Read};


// Define a custom error type
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}


// Implement `From` trait to convert `io::Error` and `ParseIntError` into `MyError`
impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

// Implement `Display` for `MyError`
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "IO error: {}", err),
            MyError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

// A function that may return an error
fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
    let mut file = File::open(path)?; // `?` propagates `io::Error` as `MyError`
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?; // `?` propagates `ParseIntError` as `MyError`
    Ok(number)
}

fn main() {
    match read_and_parse_file("numbers.txt") {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => println!("An error occurred: {}", e),
    }
}