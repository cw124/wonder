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
    /// The players in the game. Moving through the vector starting from index 0 is equivalent to moving clockwise
    /// around the table of players. The player at the end of the vector also sits next to player at index 0, of course.
    /// For example, for a game of 5 players, the player at index 1 sits to the left (ie. clockwise) of the player at
    /// index 0, and the player at index 4 sits to the right (ie. anti-clockwise) of the player at index 0.
    players: Vec<Player>,

    /// The game turn. Runs from 0 to 17 for 3 ages of 6 turns each.
    turn: u32,

    /// The discard pile. Starts empty and gains the final, unplayed card from each player at the end of each age.
    discard_pile: Vec<Card>,
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
            players,
            turn: 0,
            discard_pile: vec![]
        }
    }

    /// Executes a turn of the game. Gets an [`Action`] from each [`Player`] and updates the game state accordingly.
    pub fn do_turn(&mut self) {
        // Get all actions first.
        let actions: Vec<(&mut Player, Action)> = self.players.iter_mut().enumerate()
            .map(|(index, player)| {
                let action = player.algorithm().get_next_action(&player, index as u32);
                (player, action)
            })
            .collect();

        // Update all players "simultaneously".
        for (player, action) in actions {
            player.do_action(&action, &mut self.discard_pile);
        }

        // Pass cards.
        let num_players = self.players.len();
        let mut hand = vec![];
        for i in 0..num_players + 1 {
            let index = if self.age() == 2 {
                // In the second age, we pass cards anti-clockwise.
                num_players - i
            } else {
                // Otherwise, pass clockwise.
                i
            } % num_players;
            hand = self.players[index].swap_hand(hand);
        }

        self.turn += 1;
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Returns the current age being played: 1, 2, or 3.
    pub fn age(&self) -> u32 {
        self.turn / 6 + 1
    }
}

/// Represents an action.
/// TODO: this needs to one day record coins paid for borrowed resources.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Action {
    Build(Card),
    Wonder(Card),
    Discard(Card),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Build(card) => write!(f, "Build {}", card.to_string()),
            Action::Wonder(card) => write!(f, "Use {} to build a wonder stage", card.to_string()),
            Action::Discard(card) => write!(f, "Discard {}", card.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::random::Random;
    use std::cmp::Ordering;

    #[test]
    #[should_panic(expected = "Must have at least three players")]
    fn new_panics_if_less_than_three_players() {
        Game::new(vec![Box::new(Random {}), Box::new(Random {})]);
    }

    #[test]
    #[should_panic(expected = "Must have at most seven players")]
    fn new_panics_if_more_than_seven_players() {
        Game::new(vec![
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
            Box::new(Random {}),
        ]);
    }

    #[test]
    fn new_game_has_correct_number_of_players() {
        assert_eq!(3, Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]).player_count());
    }

    #[test]
    fn do_turn_increments_turn() {
        let mut game = Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]);
        assert_eq!(0, game.turn);
        game.do_turn();
        assert_eq!(1, game.turn);
    }

    #[test]
    fn age_updates_correctly_with_turns() {
        let game = Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]);
        assert_eq!(1, game.age());
        // TODO: fix the Random algorithm so it can guarantee to not panic
        // TODO: deal new cards at the start of the second and third ages
    }

    #[test]
    fn do_turn_rotates_hands() {
        // TODO: should probably introduce a deterministic algorithm here, especially when the Random one starts
        //  building wonders or discarding cards.
        // TODO: can this be written better in Rust? It's pretty tortuous.

        fn sorter(a: &Card, b: &Card) -> Ordering {
            a.to_string().cmp(&b.to_string())
        }

        let mut game = Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]);

        let mut player0 = game.players[0].hand().clone();
        player0.sort_by(sorter);
        let mut player1 = game.players[1].hand().clone();
        player1.sort_by(sorter);
        let mut player2 = game.players[2].hand().clone();
        player2.sort_by(sorter);

        game.do_turn();

        // Each player's new hand, plus the card built by the player on the right, should equal the player on the
        // right's original hand.

        let mut player1a = game.players[1].hand().clone();
        player1a.append(&mut game.players[0].built_structures().clone());
        player1a.sort_by(sorter);
        assert_eq!(player1a, player0);

        let mut player2a = game.players[2].hand().clone();
        player2a.append(&mut game.players[1].built_structures().clone());
        player2a.sort_by(sorter);
        assert_eq!(player2a, player1);

        let mut player0a = game.players[0].hand().clone();
        player0a.append(&mut game.players[2].built_structures().clone());
        player0a.sort_by(sorter);
        assert_eq!(player0a, player2);
    }
}
