use crate::domain::space::{Map, Point};

#[allow(dead_code)]
pub struct Rover {
    map : Map,
    current_position: Point,
}