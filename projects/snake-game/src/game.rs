use std::{
    io::{stdout, Write, Read},
    thread,
    time::Duration,
};

use rand::Rng;
use termion::async_stdin;
use termion::{input::TermRead, raw::IntoRawMode};

use crate::{direction::Direction, point::Point, snake::Snake};

pub struct Game;

enum GameCommand {
    GenSnack,
    Continue,
    Finish,
    ChangeDirection(Direction),
}

impl Game {
    pub fn run() -> Result<(), &'static str> {
        let mut snake: Snake = Default::default();
        let board = (15, 15);
        let mut snack = Self::gen_snack(&snake, &board);
        loop {
            match Self::check_conditions(&mut snake, &snack, &board)? {
                GameCommand::GenSnack => snack = Self::gen_snack(&snake, &board),
                GameCommand::Finish => break,
                GameCommand::ChangeDirection(direction) => snake.change_direction(direction),
                _ => {}
            }
            Self::gen_board(&snake, &snack, &board);
            thread::sleep(Duration::from_millis(500));
            snake.step(board)?;
        }
        Ok(())
    }

    fn read_keys(board: String) -> GameCommand {
        print!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1),  termion::cursor::Hide);
        println!("{}", board);
        
        let stdout = stdout();
        let mut stdout = stdout.into_raw_mode().unwrap();
        let mut stdin = async_stdin().bytes();
        let input = stdin.next();
        // let input = input.next();
        
        let game_command = if let Some(Ok(key)) = input {
            match key {
                b'q' => GameCommand::Finish,
                b'w' => GameCommand::ChangeDirection(Direction::Up),
                b's' => GameCommand::ChangeDirection(Direction::Down),
                b'a' => GameCommand::ChangeDirection(Direction::Left),
                b'd' => GameCommand::ChangeDirection(Direction::Right),
                _ => GameCommand::Continue,
            }
        } else {
            GameCommand::Continue
        };
        stdout.flush().unwrap();
        game_command
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

    fn check_conditions(
        snake: &mut Snake,
        snack: &Point,
        board: &(usize, usize),
    ) -> Result<GameCommand, &'static str> {
        if snake.head == *snack {
            snake.increase_snake_size();
            Ok(GameCommand::GenSnack)
        } else if snake.body.contains(&snake.head) {
            Err("game over, snake hit yourself")
        } else {
            let board = Self::gen_board(snake, snack, board);
            Ok(Self::read_keys(board))
        }
    }
}
