use crate::game::Game;

mod card;
mod wonder;
mod power;
mod resources;
mod player;
mod game;
mod utils;

fn main() {
    let game = Game::new(4);
    println!("{:#?}", game);
    println!();
    game.print_state_for_user(0);
}