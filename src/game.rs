//! Represents the whole game state.

use core::fmt;
use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use crate::card;
use crate::card::{Age, Card};
use crate::player::Player;
use crate::wonder::{WonderSide, WonderType};
use crate::algorithms::PlayingAlgorithm;

/// Represents the whole game state.
#[derive(Debug)]
pub struct Game {
    players: Vec<Player>
}

#[allow(dead_code)]
impl Game {
    /// Generates a new game with each player playing according to the given algorithm. Players will be randomly
    /// allocated wonders and dealt a random hand of first age cards. `algorithms` must have between 3 and 7 entries
    /// inclusive, corresponding to between 3 and 7 players.
    /// TODO: for now, everyone gets the A side of the wonder.
    pub fn new(algorithms: Vec<Box<dyn PlayingAlgorithm>>) -> Game {
        if algorithms.len() < 3 {
            panic!("Must have at least three players")
        }
        if algorithms.len() > 7 {
            panic!("Must have at most seven players")
        }

        let mut wonder_types: Vec<WonderType> = WonderType::iter().collect();
        wonder_types.shuffle(&mut thread_rng());

        let mut deck = card::new_deck(Age::First, algorithms.len() as u32);

        // For each player, pick a random wonder and deal seven random cards.
        let players: Vec<Player> = algorithms.into_iter()
            .zip(wonder_types)
            .map(|(algorithm, wonder_type)|
                Player::new(wonder_type, WonderSide::A, deck.drain(0..7).collect(), algorithm))
            .collect();

        Game {
            players
        }
    }

    /// Executes a turn of the game. Gets an [`Action`] from each [`Player`] and updates the game state accordingly.
    pub fn do_turn(&mut self) {
        let actions: Vec<(&mut Player, Action)> = self.players.iter_mut().enumerate()
            .map(|(index, player)| {
                let action = player.algorithm.get_next_action(&player, index as u32);
                (player, action)
            })
            .collect();

        for (player, action) in actions {
            player.do_action(&action);
        }
    }

    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }
}

/// Represents an action.
/// TODO: this needs to one day record coins paid for borrowed resources.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Action {
    Build(Card),
    Wonder(Card),
    Discard
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Build(card) => write!(f, "Build {}", card.to_string()),
            Action::Wonder(card) => write!(f, "Use {} to build a wonder stage", card.to_string()),
            Action::Discard => write!(f, "Discard"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::random::Random;

    #[test]
    #[should_panic(expected = "Must have at least three players")]
    fn new_panics_if_less_than_three_players() {
        assert_eq!(3, Game::new(vec![Box::new(Random {}), Box::new(Random {})]).get_player_count());
    }

    #[test]
    #[should_panic(expected = "Must have at most seven players")]
    fn new_panics_if_more_than_seven_players() {
        assert_eq!(3, Game::new(vec![
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
        ]).get_player_count());
    }

    #[test]
    fn new_game_has_correct_number_of_players() {
        assert_eq!(3, Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]).get_player_count());
    }
}
