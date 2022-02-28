use crate::{direction::Direction, point::Point};

pub struct Snake {
    pub head: Point,
    pub body: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub fn step(&mut self, board: (usize, usize)) -> Result<(), &'static str> {
        let previews_head_pos = self.head;
        self.move_head(&board)?;
        self.move_body(previews_head_pos);
        Ok(())
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if Direction::reverse_direction(self.direction) == direction {
            return;
        }
        self.direction = direction;
    }

    fn move_head(&mut self, board: &(usize, usize)) -> Result<(), &'static str> {
        match self.direction {
            Direction::Up if self.head.y == 0 => Err("game over, hit in top wall"),
            Direction::Down if self.head.y >= board.1 => Err("game over, hit in down wall"),
            Direction::Left if self.head.x == 0 => Err("game over, hit in left wall"),
            Direction::Right if self.head.x >= board.0 => Err("game over, hit in right wall"),
            _ => {
                self.head.transform(self.direction);
                Ok(())
            }
        }
    }

    fn move_body(&mut self, previews_head_pos: Point) {
        let body = &mut self.body;
        let mut previews_pos = previews_head_pos;
        for point in body.iter_mut() {
            std::mem::swap(&mut previews_pos, point);
        }
    }

    pub fn increase_snake_size(&mut self) {
        let last = self.body.last().unwrap().clone();
        self.body.push(last);
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7), Point::new(5, 7)],
            direction: Default::default(),
        }
    }
}

#[cfg(test)]
mod snake_tests {

    use super::*;

    #[test]
    fn move_snake_head_to_right_in_board_should_move_successful() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Right,
        };
        let expected_point = Point::new(8, 7);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    fn move_snake_head_to_left_in_board_should_move_successful() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Left,
        };
        let expected_point = Point::new(6, 7);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    fn move_snake_head_to_up_in_board_should_move_successful() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Up,
        };
        let expected_point = Point::new(7, 6);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    fn move_snake_head_to_down_in_board_should_move_successful() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Down,
        };
        let expected_point = Point::new(7, 8);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    #[should_panic(expected = "game over, hit in right wall")]
    fn move_snake_head_to_right_in_board_should_hit_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Right,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "game over, hit in left wall")]
    fn move_snake_head_to_left_in_board_should_hit_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(0, 7),
            body: vec![],
            direction: Direction::Left,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "game over, hit in down wall")]
    fn move_snake_head_to_down_in_board_should_hit_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(0, 7),
            body: vec![],
            direction: Direction::Down,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "game over, hit in top wall")]
    fn move_snake_head_to_up_in_board_should_hit_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(0, 0),
            body: vec![],
            direction: Direction::Up,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    fn move_entire_snake_to_right_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Right,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(8, 7), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn move_entire_snake_to_left_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Left,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(6, 7), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn move_entire_snake_to_up_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Up,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(7, 6), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn move_entire_snake_to_down_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Down,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(7, 8), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn dont_change_direction_if_reverse_direction() {
        assert_directions(vec![
            (Direction::Down, Direction::Up),
            (Direction::Up, Direction::Down),
            (Direction::Left, Direction::Right),
            (Direction::Right, Direction::Left),
        ])
    }

    fn assert_directions(directions: Vec<(Direction, Direction)>) {
        for (expected, change) in directions {
            let mut snake = Snake {
                head: Point::new(7, 7),
                body: vec![Point::new(6, 7)],
                direction: expected,
            };
            snake.change_direction(change);
            assert_eq!(expected, snake.direction);
        }
    }

    #[test]
    fn test_increate_size() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Default::default(),
        };
        snake.increase_snake_size();
        snake.step((15,15)).unwrap();
        assert_eq!(2, snake.body.len());
        assert_eq!(Point::new(6,7), *snake.body.last().unwrap());
    }
}
