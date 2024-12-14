use core::num;
use std::fs::File;
use std::io::{self,Read};
use std::num::ParseIntError;

fn read_file_contents(filename : &str)->Result<String, io::Error>{
    let mut file = match File::open (filename) {
        Ok(file)=>file,
        Err(e)=> return Err(e)
    };
    let mut content = String::new();
    let _ = match file.read_to_string(&mut content){
        Ok(_) => Ok::<String, io::Error>(content.clone()),
        Err(e)=>return Err(e)
    };


    Ok(content)
}

fn parse_int(value:&str)->Result<i32, ParseIntError>{
    match value.parse::<i32>(){
        Ok(num)=> Ok(num),
        Err(err)=>Err(err)
    }
}

fn main() {
    match read_file_contents("./sample.txt"){
        Ok(content)=> println!("File : {content}"),
        Err(e)=>println!("Error :{:?}", e)
    };

    match parse_int("yuyuy"){
        Ok(num)=>println!("Number,: {num}"),
        Err(e)=>println!("Error : {e}")
    }
}
