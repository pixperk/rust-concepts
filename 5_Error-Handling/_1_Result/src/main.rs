use std::fs::File;
use std::io::{self, Read};

fn read_file(filename : &str)->Result<String, io::Error>{
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn divide(a:i32, b:i32) -> Result<i32, &'static str>{
    if b== 0{
        Err("Cannot divide by zero")
    }else{
        Ok(a/b)
    }
}

fn main() {

    let result1 = divide(10, 2);
    match result1 {
        Ok(res)=>{
            println!("Answer = {res}");
        }
        Err(err)=>{
            println!("Error in division : {err}");
        }
    }

    let result1 = divide(10, 0);
    match result1 {
        Ok(res)=>{
            println!("Answer = {res}");
        }
        Err(err)=>{
            println!("Error in division : {err}");
        }
    }


    let file_result = File::open("nonexistent.txt");
    match file_result {
        Ok(_file)=>{
            println!("Successfully retreived the file");
        }
        Err(error)=>{
            println!("Error : {:?}",error);
        }
        
    }
    let file_content = read_file("example.txt");
    match file_content {
        Ok(_file)=>{
            println!("Successfully retreived the file");
        }
        Err(error)=>{
            println!("Error : {:?}",error);
        }
}
}

