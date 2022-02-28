use snake_game::point::Point;

fn main() {
    let point = Point::new(7, 7);
    let (x, y) = (15, 15);
    for x in 0..x {
        for y in 0..y {
            if point == (x, y) {
                print!("# ")
            }  else {
                print!("- ");   
            }
        }
        println!();
    }
}
