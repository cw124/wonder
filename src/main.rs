use crate::algorithms::human::Human;
use crate::algorithms::monte_carlo::MonteCarlo;
use crate::algorithms::random::Random;
use crate::game::Game;
use crate::utils::plural;
use itertools::Itertools;

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
    let mut game = Game::new(vec![Box::new(Human {}), Box::new(MonteCarlo {}), Box::new(Random {})]);
    let scores = game.play();
    let sorted_scores: Vec<(usize, i32)> = scores
        .into_iter()
        .enumerate()
        .sorted_by_key(|(_, score)| *score)
        .rev()
        .collect();

    // TODO: deal with draws
    println!("Player {} wins!", sorted_scores[0].0 + 1);
    println!();
    for (i, score) in sorted_scores {
        println!("Player {}: {}", i + 1, plural(score, "point"));
    }
}
