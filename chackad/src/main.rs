mod board;
mod game;
mod piece;
use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
