use crate::domain::space::{Direction, Map, Point};
use crate::domain::rover::Errors::InvalidMove;
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Errors {
    InvalidMove
}
#[allow(dead_code)]
pub struct Rover {
    map : Map,
    current_position: Point,
    direction : Direction,
}

impl Rover {
    pub fn new(map: Map, current_position: Point, direction: Direction) -> Self {
        Self { map, current_position, direction }
    }
    fn move_to(&mut self, d: Direction) -> Result<(), Errors> {
        let next_point:Point = self.get_next_point(d);
        if self.map.can_move_to(next_point.clone()) {
            self.current_position = next_point;
            return Ok(())
        }

        Err(InvalidMove)
    }

    fn get_next_point(&self, d: Direction) -> Point {
        match d {
            Direction::N => {Point::new(self.current_position.x, self.current_position.y+1)}
            Direction::S => {Point::new(self.current_position.x, self.current_position.y-1)}
            Direction::E => {Point::new(self.current_position.x+1, self.current_position.y)}
            Direction::W => {Point::new(self.current_position.x-1, self.current_position.y)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_to_point() {
        let obstacle_point = Point::new(1, 1);
        let starting_point = Point::new(1, 0);

        let map = Map::new(2, 2, vec![obstacle_point.clone()]);
        let mut rover = Rover::new(map, starting_point, Direction::N);

        assert_eq!(rover.move_to(Direction::N), Err(InvalidMove));
        assert_eq!(rover.move_to(Direction::S), Ok(()));
    }
}