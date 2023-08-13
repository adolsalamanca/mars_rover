
pub struct Map {
    upper_left_limit: Point,
    upper_right_limit: Point,
    lower_left_limit: Point,
    lower_right_limit: Point,
    obstacles: Vec<Point>,
}

impl Map {
    pub fn new(length_dimension: i64, width_dimension: i64, obstacles:Vec<Point>) -> Self {
        Self {
            upper_left_limit: Point {
                x: -width_dimension,
                y: length_dimension,
            },
            upper_right_limit: Point {
                x: width_dimension,
                y: length_dimension,
            },
            lower_left_limit: Point {
                x: -width_dimension,
                y: -length_dimension,
            },
            lower_right_limit: Point {
                x: width_dimension,
                y: -length_dimension,
            },

            obstacles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_map() {
        let p = Map::new(1, 1, vec![]);

        assert_eq!(Point{x:1, y:1}, p.upper_right_limit);
        assert_eq!(Point{x:-1, y:1}, p.upper_left_limit);
        assert_eq!(Point{x:1, y:-1}, p.lower_right_limit);
        assert_eq!(Point{x:-1, y:-1}, p.lower_left_limit);
    }
}



#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}