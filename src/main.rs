use crate::domain::space::Map;
use std::io;
use std::process::exit;

mod domain;

fn main() {
    let mut input = String::new();
    println!("Enter planet height:");
    let length:i64 = read_number(&mut input);

    println!("Enter planet width:");
    let width:i64 = read_number(&mut input);

    let p = Map::new(length, width, vec![]);
    println!("{:?}", p)
}

fn read_number(input: &mut String) -> i64{
    match io::stdin().read_line(input){
        Ok(number) => number,
        Err(_) => {
            eprintln!("error: cannot read the line");
            exit(-1);
        }
    };

    let x:i64 = match input.trim().parse(){
        Ok(number) => number,
        Err(_) => {
            eprintln!("error: invalid value for the coordinate, it needs to be a number");
            exit(-1);
        }
    };

    input.clear();
    x
}