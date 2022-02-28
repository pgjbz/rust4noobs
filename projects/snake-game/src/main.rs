use snake_game::game::Game;

fn main() {
    if let Err(msg) = Game::run() {
        eprintln!("{}", msg)
    }
}
