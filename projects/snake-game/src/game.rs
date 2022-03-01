use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use rand::Rng;
use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode};

use crate::{direction::Direction, point::Point, snake::Snake};

pub struct Game;

impl Game {
    pub fn run() -> Result<(), &'static str> {
        let mut snake: Snake = Default::default();
        let board = (15, 15);
        let mut snack = Self::gen_snack(&snake, &board);
        let mut stdin = termion::async_stdin().keys();
        loop {
            if snake.head == snack {
                snake.increase_snake_size();
                snack = Self::gen_snack(&snake, &board);
            } else if snake.body.contains(&snake.head) {
                return Err("game over, snake hit yourself");
            }
            let board_game = Self::gen_board(&snake, &snack, &board);
            print!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide
            );
            println!("{}", board_game);
            let stdout = io::stdout().into_raw_mode().unwrap();
            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    Key::Char('a') | Key::Left => snake.change_direction(Direction::Left),
                    Key::Char('w') | Key::Up => snake.change_direction(Direction::Up),
                    Key::Char('s') | Key::Down => snake.change_direction(Direction::Down),
                    Key::Char('d') | Key::Right => snake.change_direction(Direction::Right),
                    key => println!("{:?}", key),
                }
            }
            stdout.lock().flush().unwrap();
            thread::sleep(Duration::from_millis(500));
            snake.step(board)?;
        }
    }

    fn gen_snack(snake: &Snake, board: &(usize, usize)) -> Point {
        let mut snack;
        loop {
            let x = rand::thread_rng().gen_range(0..=board.0 - 1);
            let y = rand::thread_rng().gen_range(0..=board.1 - 1);
            snack = Point::new(x, y);
            if snake.head != snack && !snake.body.contains(&snack) {
                break;
            }
        }
        snack
    }

    fn gen_board(snake: &Snake, snack: &Point, board: &(usize, usize)) -> String {
        let mut buffer = String::new();
        for y in 0..board.1 {
            for x in 0..board.0 {
                if snake.head == (x, y) {
                    buffer.push_str("0 ")
                } else if snake.body.contains(&Point::new(x, y)) {
                    buffer.push_str("# ");
                } else if *snack == (x, y) {
                    buffer.push_str("+ ");
                } else {
                    buffer.push_str("- ");
                }
            }
            buffer.push('\n');
        }
        buffer
    }
}
