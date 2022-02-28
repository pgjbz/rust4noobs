use snake_game::{direction::Direction, point::Point, snake::Snake};

fn main() {
    let mut snake: Snake = Default::default();
    let board = (15, 15);
    print_board(&snake, board);
    println!("after move");
    snake.step(board).unwrap();
    println!("after move");
    print_board(&snake, board);
    snake.change_direction(Direction::Up);
    println!("after move");
    snake.step(board).unwrap();
    println!("after move");
    print_board(&snake, board);
    snake.step(board).unwrap();
    println!("after move");
    print_board(&snake, board);
    snake.change_direction(Direction::Left);
    snake.step(board).unwrap();
    println!("after move");
    print_board(&snake, board);
    snake.step(board).unwrap();
    println!("after move");
    print_board(&snake, board);
}

fn print_board(snake: &Snake, board: (usize, usize)) {
    for y in 0..board.1 {
        for x in 0..board.0 {
            if snake.head == (x, y) {
                print!("0 ")
            } else if snake.body.contains(&Point::new(x, y)) {
                print!("# ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
}
