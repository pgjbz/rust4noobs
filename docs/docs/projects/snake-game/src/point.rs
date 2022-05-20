use crate::direction::Direction;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn transform(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn move_to_up() {
        let mut point = Point::new(1, 1);
        point.transform(Direction::Up);
        assert_eq!(Point::new(1, 0), point);
    }

    #[test]
    fn move_to_down() {
        let mut point = Point::new(1, 1);
        point.transform(Direction::Down);
        assert_eq!(Point::new(1, 2), point);
    }

    #[test]
    fn move_to_right() {
        let mut point = Point::new(1, 1);
        point.transform(Direction::Right);
        assert_eq!(Point::new(2, 1), point);
    }

    #[test]
    fn move_to_left() {
        let mut point = Point::new(1, 1);
        point.transform(Direction::Left);
        assert_eq!(Point::new(0, 1), point);
    }
}
