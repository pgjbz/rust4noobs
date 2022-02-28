use rand::Rng;
use snake_game::{point::Point, snake::Snake};

fn main() {
    let snake: Snake = Default::default();
    let board = (15, 15);
    let snack = gen_snack(&snake, &board);
    print_board(&snake, snack, board);
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

fn print_board(snake: &Snake, snack: Point, board: (usize, usize)) {
    for y in 0..board.1 {
        for x in 0..board.0 {
            if snake.head == (x, y) {
                print!("0 ")
            } else if snake.body.contains(&Point::new(x, y)) {
                print!("# ");
            } else if snack == (x, y) {
                print!("+ ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
}
