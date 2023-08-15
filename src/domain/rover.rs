use crate::domain::space::{Direction, Map, Movement, Point};
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

    fn move_to(&mut self, m: Movement) -> Result<(), Errors> {
        let next_point:Point = self.get_next_point(m);
        if self.map.can_move_to(next_point.clone()) {
            self.current_position = next_point;
            return Ok(())
        }

        Err(InvalidMove)
    }

    fn get_next_point(&self, m: Movement) -> Point {
        match m {
            Movement::F => {Point::new(self.current_position.x, self.current_position.y+1)}
            Movement::B => {Point::new(self.current_position.x, self.current_position.y-1)}
            Movement::R => {Point::new(self.current_position.x+1, self.current_position.y)}
            Movement::L => {Point::new(self.current_position.x-1, self.current_position.y)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_fail_moving_to_an_obstacle() {
        let obstacle_point = Point::new(1, 1);
        let starting_point = Point::new(1, 0);

        let map = Map::new(2, 2, vec![obstacle_point.clone()]);
        let mut rover = Rover::new(map, starting_point, Direction::N);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
    }

    #[test]
    fn should_fail_moving_to_an_edge() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::N);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
    }

    #[test]
    fn should_fail_moving_to_an_edge_backwards() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::S);

        assert_eq!(rover.move_to(Movement::B), Err(InvalidMove));
    }

    #[test]
    fn should_succeed_moving_to_an_allowed_position() {
        let obstacle_point = Point::new(1, 1);
        let starting_point = Point::new(1, 0);

        let map = Map::new(2, 2, vec![obstacle_point.clone()]);
        let mut rover = Rover::new(map, starting_point, Direction::N);

        assert_eq!(rover.move_to(Movement::B), Ok(()));
    }
}