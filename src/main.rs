use crate::game::Game;

mod card;
mod wonder;
mod power;
mod resources;
mod player;
mod game;
mod utils;
mod table;
mod algorithms;

fn main() {
    let mut game = Game::new(4);
    loop {
        println!();
        game.do_turn();
    }
}