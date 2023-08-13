use crate::domain::space::Point;
use std::io;
use std::process::exit;

mod domain;

fn main() {
    let mut input = String::new();
    println!("Enter x coordinate:");
    let x:i64 = read_coordinate(&mut input);

    println!("Enter y coordinate:");
    let y:i64 = read_coordinate(&mut input);

    let p = create_point(x,y);
    println!("{:?}", p)
}

fn read_coordinate(input: &mut String) -> i64{
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

fn create_point(x: i64, y: i64) -> Point{
    Point::new(x,y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point() {
        let p = create_point(1,2);

        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
    }
}
