use crate::algorithms::human::Human;
use crate::algorithms::random::Random;
use crate::game::Game;

mod action;
mod algorithms;
mod card;
mod game;
mod player;
mod power;
mod resources;
mod table;
mod utils;
mod wonder;

fn main() {
    let mut game = Game::new(vec![Box::new(Human {}), Box::new(Random {}), Box::new(Random {})]);
    loop {
        println!();
        game.do_turn();
    }
}
