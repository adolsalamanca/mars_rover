use crate::domain::space::{Map, Point};
use rand::Rng;
use application::input;

mod domain;
mod application;

fn main() {
    let mut input = String::new();
    println!("Enter planet height:");
    let length:i64 = input::read_number(&mut input);

    println!("Enter planet width:");
    let width:i64 = input::read_number(&mut input);

    println!("Enter number of obstacles:");
    let number_of_obstacles:i64 = input::read_number(&mut input);
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