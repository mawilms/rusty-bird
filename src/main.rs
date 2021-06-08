mod game;

fn main() {
    game::Game::start().expect("Error while staring game");
}
