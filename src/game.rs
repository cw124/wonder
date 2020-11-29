use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use crate::player::Player;
use crate::wonder::{WonderSide, WonderType};
use crate::card;
use crate::card::Age;

/// Represents the whole game state.
#[derive(Debug)]
pub struct Game {
    players: Vec<Player>
}

#[allow(dead_code)]
impl Game {
    /// Generates a new game with the given number of players. Players will be randomly allocated wonders and dealt a
    /// random hand of first age cards.
    /// TODO: for now, everyone gets the A side of the wonder.
    pub fn new(player_count: u32) -> Game {
        let mut wonder_types: Vec<WonderType> = WonderType::iter().collect();
        wonder_types.shuffle(&mut thread_rng());

        let mut deck = card::new_deck(Age::First, player_count);

        // Pick a random wonder each, and deal seven random cards to each player.
        let players: Vec<Player> = wonder_types
            .drain(0..player_count as usize)
            .map(|wonder_type| Player::new(wonder_type, WonderSide::A, deck.drain(0..7).collect()))
            .collect();

        Game {
            players
        }
    }

    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_has_correct_number_of_players() {
        assert_eq!(3, Game::new(3).get_player_count());
    }
}
