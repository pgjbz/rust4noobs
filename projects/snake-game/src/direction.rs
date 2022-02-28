#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reverse_direction(other: Self) -> Self {
        match other {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[cfg(test)]
mod direction_tests {
    use crate::direction::Direction;

    #[test]
    fn test_reverse_right_direction() {
        assert_eq!(
            Direction::Left,
            Direction::reverse_direction(Direction::Right)
        )
    }

    #[test]
    fn test_reverse_left_direction() {
        assert_eq!(
            Direction::Right,
            Direction::reverse_direction(Direction::Left)
        )
    }

    #[test]
    fn test_reverse_down_direction() {
        assert_eq!(Direction::Up, Direction::reverse_direction(Direction::Down))
    }

    #[test]
    fn test_reverse_up_direction() {
        assert_eq!(Direction::Down, Direction::reverse_direction(Direction::Up))
    }
}
