use crate::domain::space::{Direction, Map, Point};

#[allow(dead_code)]
pub struct Rover {
    map : Map,
    current_position: Point,
    direction : Direction,
}