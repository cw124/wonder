use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use table::Table;

use crate::{card, table};
use crate::card::Age;
use crate::player::Player;
use crate::wonder::{WonderSide, WonderType};

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

    /// Prints out the current game state for the given user index.
    pub fn print_state_for_user(&self, player: u32) {
        let player = &self.players[player as usize];

        let mut hand = Table::new(vec![String::from("Card"), String::from("Cost"), String::from("Power")]);
        player.hand.iter()
            .map(|card| vec![card.to_string(), card.cost().to_string(), card.power().to_string()])
            .for_each(|row| hand.add(row));

        let mut played = Table::new(vec![String::from("Card"), String::from("Power")]);
        player.built_structures.iter()
            .map(|card| vec![card.to_string(), card.power().to_string()])
            .for_each(|row| played.add(row));

        println!("Wonder: {} (side {:?})", player.wonder.wonder_type.name(), player.wonder.wonder_side);
        println!("Coins: {}", player.coins);
        println!();
        println!("Hand:");
        hand.print("  ", 4);
        println!();
        println!("Played:");
        played.print("  ", 4);

        // TODO: show wonder stages and which have been built
        // TODO: show chained cards in cost column
        // TODO: show everyone else's wonders, coins, and played cards
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
