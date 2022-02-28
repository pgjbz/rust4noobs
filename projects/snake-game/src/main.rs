use snake_game::{point::Point, snake::Snake};

fn main() {
    print_board(&Snake::default())
}

fn print_board(snake: &Snake) {
    let (x, y) = (15, 15);
    for y in 0..y {
        for x in 0..x {
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