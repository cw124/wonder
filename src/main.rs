use crate::game::Game;

mod card;
mod wonder;
mod power;
mod resources;
mod player;
mod game;
mod utils;
mod table;

fn main() {
    let game = Game::new(4);
    let player_move = game.ask_for_move(0);
    println!("You chose to {:?}", player_move);
}