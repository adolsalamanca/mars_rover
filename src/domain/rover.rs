use crate::domain::space::{Direction, Map, Movement, Point};
use crate::domain::rover::Errors::*;
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Errors {
    InvalidMove,
    WrongInstructions,
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
        let (next_point, direction) = self.compute_command(m);
        if self.is_point_reachable(next_point.clone()) {
            self.current_position = next_point;
            self.direction = direction;
            return Ok(())
        }

        Err(InvalidMove)
    }

    fn compute_command(&mut self, m: Movement) -> (Point, Direction) {
                match self.direction {
                    Direction::S => {
                        match m {
                            Movement::F => {
                                (Point::new(self.current_position.x, self.current_position.y-1), Direction::S)
                            },
                            Movement::B => {
                                (Point::new(self.current_position.x, self.current_position.y+1), Direction::N)
                            },
                            Movement::R => {
                                (Point::new(self.current_position.x-1, self.current_position.y), Direction::W)
                            },
                            Movement::L => {
                                (Point::new(self.current_position.x+1, self.current_position.y), Direction::E)
                            },
                        }
                    },
                    Direction::N => {
                        match m {
                            Movement::F => {
                                (Point::new(self.current_position.x, self.current_position.y+1), Direction::N)
                            },
                            Movement::B => {
                                (Point::new(self.current_position.x, self.current_position.y-1), Direction::S)
                            },
                            Movement::R => {
                                (Point::new(self.current_position.x+1, self.current_position.y), Direction::E)
                            },
                            Movement::L => {
                                (Point::new(self.current_position.x-1, self.current_position.y), Direction::W)
                            },
                        }
                    },
                    Direction::E => {
                        match m {
                            Movement::F => {
                                (Point::new(self.current_position.x+1, self.current_position.y), Direction::E)
                            },
                            Movement::B => {
                                (Point::new(self.current_position.x-1, self.current_position.y), Direction::W)
                            },
                            Movement::R => {
                                (Point::new(self.current_position.x, self.current_position.y-1), Direction::S)
                            },
                            Movement::L => {
                                (Point::new(self.current_position.x, self.current_position.y+1), Direction::N)
                            },
                        }
                    },
                    Direction::W => {
                        match m {
                            Movement::F => {
                                (Point::new(self.current_position.x-1, self.current_position.y), Direction::W)
                            },
                            Movement::B => {
                                (Point::new(self.current_position.x+1, self.current_position.y), Direction::E)
                            },
                            Movement::R => {
                                (Point::new(self.current_position.x, self.current_position.y+1), Direction::N)
                            },
                            Movement::L => {
                                (Point::new(self.current_position.x, self.current_position.y-1), Direction::S)
                            },
                        }
                    },
                }
    }

    pub fn follow_instructions(&mut self, instructions: Vec<Movement>) -> Result<Point, Errors> {
        for i in instructions {
            let res= self.move_to(i);
            match res {
                Ok(_) => {},
                Err(_) => return Err(WrongInstructions),
            }
        }

        Ok(self.current_position.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_follow_orders_successfully(){
        let starting_point = Point::new(0, 0);
        let map = Map::new(2, 2, vec![Point::new(1, 1)]);

        let mut rover = Rover::new(map, starting_point, Direction::N);
        let mut instructions:Vec<Movement> = Vec::new();
        instructions.push(Movement::F);
        instructions.push(Movement::L);

        let result:Result<Point, Errors> = rover.follow_instructions(instructions);
        let expected_result = Ok(Point::new(-1, 1));

        assert_eq!(expected_result, result)
    }

    #[test]
    fn should_stop_after_a_wrong_order(){
        let starting_point = Point::new(0, 0);
        let map = Map::new(2, 2, vec![Point::new(-1, 1)]);

        let mut rover = Rover::new(map, starting_point, Direction::N);
        let mut instructions:Vec<Movement> = Vec::new();
        instructions.push(Movement::F);
        instructions.push(Movement::L);

        let result:Result<Point, Errors> = rover.follow_instructions(instructions);
        let expected_result = Err(WrongInstructions);

        assert_eq!(expected_result, result)
    }

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
        assert_eq!(Direction::N, rover.direction);
    }

    #[test]
    fn should_fail_moving_to_an_edge() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::N);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
        assert_eq!(Direction::N, rover.direction);
    }

    #[test]
    fn should_fail_moving_to_an_edge_backwards() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::S);

        assert_eq!(rover.move_to(Movement::B), Err(InvalidMove));
        assert_eq!(Direction::S, rover.direction);
    }
    #[test]
    fn should_fail_moving_forward_to_an_edge_being_east_oriented() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::E);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
        assert_eq!(Direction::E, rover.direction);
    }
    #[test]
    fn should_fail_moving_left_to_an_edge_being_east_oriented() {
        let starting_point = Point::new(2, 2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::E);

        assert_eq!(rover.move_to(Movement::L), Err(InvalidMove));
        assert_eq!(Direction::E, rover.direction);
    }
    #[test]
    fn should_fail_moving_left_to_an_edge_being_west_oriented() {
        let starting_point = Point::new(-2, -2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::W);

        assert_eq!(rover.move_to(Movement::L), Err(InvalidMove));
        assert_eq!(Direction::W, rover.direction);
    }
    #[test]
    fn should_fail_moving_forward_to_an_edge_being_west_oriented() {
        let starting_point = Point::new(-2, -2);

        let map = Map::new(2, 2, vec![]);
        let mut rover = Rover::new(map, starting_point, Direction::W);

        assert_eq!(rover.move_to(Movement::F), Err(InvalidMove));
        assert_eq!(Direction::W, rover.direction);
    }

    #[test]
    fn should_succeed_moving_to_an_allowed_position() {
        let obstacle_point = Point::new(1, 1);
        let starting_point = Point::new(1, 0);

        let map = Map::new(2, 2, vec![obstacle_point.clone()]);
        let mut rover = Rover::new(map, starting_point, Direction::N);

        assert_eq!(rover.move_to(Movement::B), Ok(()));
        assert_eq!(Direction::S, rover.direction);
    }
}