#[allow(dead_code)]
pub enum Direction {
    N,
    S,
    E,
    W
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Movement {
    F,
    B,
    R,
    L
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
    pub fn new(length_dimension: u64, width_dimension: u64, obstacles:Vec<Point>) -> Self {
        Self {
            upper_left_limit: Point::new(-(width_dimension as i64), length_dimension as i64),
            upper_right_limit: Point::new(width_dimension as i64, length_dimension as i64),
            lower_left_limit: Point::new(-(width_dimension as i64), -(length_dimension as i64)),
            lower_right_limit: Point::new(width_dimension as i64, -(length_dimension as i64)),

            obstacles,
        }
    }

    pub fn is_obstacles_free(&self, p: Point) -> bool {
        !self.obstacles.contains(&p)
    }

    pub fn is_inside_limits(&self, p:Point) -> bool {
        if p.x > self.upper_right_limit.x || p.x < self.lower_left_limit.x {
            return false
        }
        if p.y > self.upper_left_limit.y || p.y < self.lower_right_limit.y {
            return false
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_map_given_dimensions() {
        let m = Map::new(1, 1, vec![]);

        assert_eq!(Point{x:1, y:1}, m.upper_right_limit);
        assert_eq!(Point{x:-1, y:1}, m.upper_left_limit);
        assert_eq!(Point{x:1, y:-1}, m.lower_right_limit);
        assert_eq!(Point{x:-1, y:-1}, m.lower_left_limit);
    }

    #[test]
    fn should_find_obstacles_when_present() {
        let obstacle_point = Point::new(1, 1);
        let map = Map::new(2, 2, vec![obstacle_point.clone()]);
        let point = Point::new(1, 2);

        assert_eq!(false, map.is_obstacles_free(obstacle_point));
        assert_eq!(true, map.is_obstacles_free(point));
    }

    #[test]
    fn should_identify_properly_map_limits() {
        let m = Map::new(2, 2, vec![]);
        let inside_limits_point = Point::new(1, 1);
        let outside_limits_point = Point::new(3, 2);
        let outside_limits_point_b = Point::new(2, 3);
        let outside_limits_point_c = Point::new(-2, -3);
        let outside_limits_point_d = Point::new(-3, -2);

        assert_eq!(true, m.is_inside_limits(inside_limits_point));
        assert_eq!(false, m.is_inside_limits(outside_limits_point));
        assert_eq!(false, m.is_inside_limits(outside_limits_point_b));
        assert_eq!(false, m.is_inside_limits(outside_limits_point_c));
        assert_eq!(false, m.is_inside_limits(outside_limits_point_d));
    }

    #[test]
    fn should_follow_orders_successfully(){

    }

    #[test]
    fn should_stop_after_a_wrong_order(){

    }
}
