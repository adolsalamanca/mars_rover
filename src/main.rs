use crate::domain::space::{Map, Point};
use std::io;
use std::process::exit;
use rand::Rng;
mod domain;

fn main() {
    let mut input = String::new();
    println!("Enter planet height:");
    let length:i64 = read_number(&mut input);

    println!("Enter planet width:");
    let width:i64 = read_number(&mut input);

    println!("Enter number of obstacles:");
    let number_of_obstacles:i64 = read_number(&mut input);
    if number_of_obstacles < 0 {
        eprintln!("error: obstacles need to be equal or higher than 0");
        return
    }

    let mut obstacles:Vec<Point> = Vec::new();
    let mut random_generator = rand::thread_rng();
    for _ in 0..number_of_obstacles {
        let x = random_generator.gen_range(-width..width+1);
        let y = random_generator.gen_range(-length..length+1);
        obstacles.push(Point::new(x,y))
    }

    let p = Map::new(length, width, obstacles);
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