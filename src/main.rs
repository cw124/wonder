use crate::game::Game;
use crate::algorithms::human::Human;
use crate::algorithms::random::Random;

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
    let mut game = Game::new(vec![Box::new(Human {}), Box::new(Random {}), Box::new(Random {})]);
    loop {
        println!();
        game.do_turn();
    }
}