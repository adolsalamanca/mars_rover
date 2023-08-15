
#[allow(dead_code)]
#[derive(Debug)]
pub struct Map {
    upper_left_limit: Point,
    upper_right_limit: Point,
    lower_left_limit: Point,
    lower_right_limit: Point,
    obstacles: Vec<Point>,
}
#[allow(dead_code)]
impl Map {
    pub fn new(length_dimension: i64, width_dimension: i64, obstacles:Vec<Point>) -> Self {
        Self {
            upper_left_limit: Point::new(-width_dimension, length_dimension),
            upper_right_limit: Point::new(width_dimension, length_dimension),
            lower_left_limit: Point::new(-width_dimension, -length_dimension),
            lower_right_limit: Point::new(width_dimension, -length_dimension),

            obstacles,
        }
    }

    pub fn is_obstacle_present(&self, p: Point) -> bool {
        return self.obstacles.contains(&p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_map() {
        let m = Map::new(1, 1, vec![]);

        assert_eq!(Point{x:1, y:1}, m.upper_right_limit);
        assert_eq!(Point{x:-1, y:1}, m.upper_left_limit);
        assert_eq!(Point{x:1, y:-1}, m.lower_right_limit);
        assert_eq!(Point{x:-1, y:-1}, m.lower_left_limit);
    }

    #[test]
    fn test_is_obstacle_present_return_true_if_there_is() {
        let obstacle_point = Point::new(1, 1);
        let m = Map::new(2, 2, vec![obstacle_point.clone()]);

        let point = Point::new(1, 2);
        assert_eq!(true, m.is_obstacle_present(obstacle_point));
        assert_eq!(false, m.is_obstacle_present(point));
    }
}



#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}