use std::{eprintln, io};
use std::process::exit;

pub fn read_number(input: &mut String) -> i64{
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
