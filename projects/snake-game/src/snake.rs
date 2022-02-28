use crate::point::Point;

pub struct Snake {
    pub head: Point,
    pub body: Vec<Point>,
}

impl Default for Snake {
    fn default() -> Self {
        Self { head: Point::new(7, 7), body: vec![
            Point::new(6,7),
            Point::new(5,7),
        ] }
    }
}

