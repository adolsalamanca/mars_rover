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

    pub fn is_point_reachable(&self, p: Point) -> bool {
        self.map.is_inside_limits(p.clone()) && self.map.is_obstacles_free(p)
    }

    fn move_to(&mut self, m: Movement) -> Result<(), Errors> {
        let next_point:Point = self.compute_command_to_coordinate(m);
        if self.is_point_reachable(next_point.clone()) {
            self.current_position = next_point;
            return Ok(())
        }

        Err(InvalidMove)
    }

    fn compute_command_to_coordinate(&self, m: Movement) -> Point {

                match self.direction {
                    Direction::S => {
                        match m {
                            Movement::F => Point::new(self.current_position.x, self.current_position.y-1),
                            Movement::B => Point::new(self.current_position.x, self.current_position.y+1),
                            Movement::R => Point::new(self.current_position.x-1, self.current_position.y),
                            Movement::L => Point::new(self.current_position.x+1, self.current_position.y),
                        }
                    },
                    Direction::N => {
                        match m {
                            Movement::F => Point::new(self.current_position.x, self.current_position.y+1),
                            Movement::B => Point::new(self.current_position.x, self.current_position.y-1),
                            Movement::R => Point::new(self.current_position.x+1, self.current_position.y),
                            Movement::L => Point::new(self.current_position.x-1, self.current_position.y),
                        }
                    },
                    Direction::E => {
                        match m {
                            Movement::F => Point::new(self.current_position.x+1, self.current_position.y),
                            Movement::B => Point::new(self.current_position.x-1, self.current_position.y),
                            Movement::R => Point::new(self.current_position.x, self.current_position.y-1),
                            Movement::L => Point::new(self.current_position.x, self.current_position.y+1),
                        }
                    },
                    Direction::W => {
                        match m {
                            Movement::F => Point::new(self.current_position.x-1, self.current_position.y),
                            Movement::B => Point::new(self.current_position.x+1, self.current_position.y),
                            Movement::R => Point::new(self.current_position.x, self.current_position.y+1),
                            Movement::L => Point::new(self.current_position.x, self.current_position.y-1),
                        }
                    },
                }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_confirm_if_point_is_reachable() {
        let starting_point = Point::new(1, 0);
        let obstacle_point = Point::new(1, 1);
        let map = Map::new(2, 2, vec![obstacle_point.clone()]);

        let rover = Rover::new(map, starting_point, Direction::N);
        let outside_map_point = Point::new(3, 2);
        let legal_map_point = Point::new(1, 2);

        assert_eq!(false, rover.is_point_reachable(obstacle_point));
        assert_eq!(false, rover.is_point_reachable(outside_map_point));
        assert_eq!(true, rover.is_point_reachable(legal_map_point));
    }

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
    fn should_fail_moving_forward_to_and_edge_being_east_oriented() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::E);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
    }
    #[test]
    fn should_fail_moving_left_to_and_edge_being_east_oriented() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::E);

        assert_eq!(rover.move_to(Movement::L), Err(InvalidMove));
    }
    #[test]
    fn should_fail_moving_left_to_and_edge_being_west_oriented() {
        let starting_point = Point::new(-2, -2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::W);

        assert_eq!(rover.move_to(Movement::L), Err(InvalidMove));
    }
    #[test]
    fn should_fail_moving_forward_to_and_edge_being_west_oriented() {
        let starting_point = Point::new(-2, -2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::W);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
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