use crate::domain::space::Point;
use std::io;

mod domain;

fn main() {
    let mut input = String::new();
    println!("Enter x coordinate:");
    io::stdin().read_line(&mut input).unwrap();
    let x:i64 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter y coordinate:");
    io::stdin().read_line(&mut input).unwrap();
    let y:i64 = input.trim().parse().unwrap();

    let p = create_point(x,y);

    println!("{:?}", p)
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
