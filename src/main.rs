use std::io;
use rand::prelude::ThreadRng;
use crate::domain::space::{Direction, Map, Point};
use rand::Rng;
use application::input;
use crate::domain::rover::Rover;

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

    // Formula to check max_obstacles given length and width for 2D maps
    // then we subtract one as starting position shouldn't have an obstacle
    let max_obstacles = (length + length + 1) * (width + width + 1) - 1;
    if number_of_obstacles > max_obstacles {
        return Err("error: the number of obstacles cannot be equal to the total amount of rows")
    }

    let starting_point = Point::new(0, 0);
    let mut obstacles:Vec<Point> = Vec::new();
    let mut random_generator = rand::thread_rng();
    for _ in 0..number_of_obstacles {
        let mut p = generate_random_point(length, width, &mut random_generator);
        while obstacles.contains(&p) || p.eq(&starting_point.clone()){
            p  = generate_random_point(length, width, &mut random_generator);
        }

        obstacles.push(p)
    }

    let map = Map::new(length, width, obstacles);
    let mut rover = Rover::new(map.clone(), starting_point, Direction::N);

    let input = io::stdin().lock();
    println!("Now enter the commands to move the rover, separated by comas, valid commands: F,B,L,R");
    let res = input::read_commands(input);

    let commands = match res {
        Ok(value) => value,
        Err(_) => {return Err("error: could not read movements")}
    };
    
    let res = rover.follow_instructions(commands);
    match res {
        Ok(p) => {
            println!("Cool, the rover arrived to destiny: \n {:?}", p);
        }
        Err(_) => {
            println!("Couldn't move to that position, last valid rover state: {:?}",rover.last_position());
            println!("Map dimensions & Obstacles: \n {:?}", map);
        }
    }


    Ok(())
}

fn generate_random_point(length: u64, width: u64, random_generator: &mut ThreadRng) -> Point {
    let x = random_generator.gen_range(-(width as i64)..width as i64 + 1);
    let y = random_generator.gen_range(-(length as i64)..length as i64 + 1);
    Point::new(x, y)
}