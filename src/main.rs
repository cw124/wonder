use crate::game::Game;

mod card;
mod wonder;
mod power;
mod resources;
mod player;
mod game;

fn main() {
    println!("{:?}", Game::new(4));
}
