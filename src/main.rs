use std::io;
use crate::domain::space::{Map, Point};
use rand::Rng;
use application::input;

mod domain;
mod application;

fn main() -> Result<(), &'static str>{
    let input = io::stdin().lock();
    println!("Enter planet length:");
    let res = input::read_number(input);
    let length = match res {
        Ok(value) => value,
        Err(_) => {return Err("error: could not read number")}
    };

    let input = io::stdin().lock();
    println!("Enter planet width:");
    let res = input::read_number(input);
    let width = match res {
        Ok(value) => value,
        Err(_) => {return Err("error: could not read number")}
    };

    let input = io::stdin().lock();
    println!("Enter the number of obstacles:");
    let res = input::read_number(input);
    let number_of_obstacles = match res {
        Ok(value) => value,
        Err(_) => {return Err("error: could not read number")}
    };

    let mut obstacles:Vec<Point> = Vec::new();
    let mut random_generator = rand::thread_rng();
    for _ in 0..number_of_obstacles {
        let x = random_generator.gen_range(-(width as i64)..width as i64+1);
        let y = random_generator.gen_range(-(length as i64)..length as i64+1);
        obstacles.push(Point::new(x,y))
    }

    let p = Map::new(length, width, obstacles);
    println!("{:?}", p);

    Ok(())
}