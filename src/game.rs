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

    /// Prints out the current game state for the given user index.
    pub fn print_state_for_user(&self, player: u32) {
        let player = &self.players[player as usize];
        println!("Wonder: {} (side {:?})", player.wonder.wonder_type.name(), player.wonder.wonder_side);
        // TODO: show wonder stages and which have been built

        println!("Coins: {}", player.coins);

        // Get the hand data up front so we can work out how wide each column needs to be.
        let hand_data: Vec<[String; 3]> = player.hand.iter()
            .map(|card| [format!("{}", card), format!("{}", card.cost()), format!("{}", card.power())])
            .collect();
        let widths: Vec<usize> = (0..2)
            .map(|i| hand_data.iter().map(|d| d[i].len()).max().unwrap_or(0) + 4)
            .collect();

        println!("Hand:");
        println!("  {:name_width$} {:cost_width$} Power", "Card", "Cost", name_width=widths[0], cost_width=widths[1]);
        println!("  {:name_width$} {:cost_width$} =====", "====", "====", name_width=widths[0], cost_width=widths[1]);
        for data in hand_data {
            println!("  {:name_width$} {:cost_width$} {}",
                     data[0], data[1], data[2], name_width=widths[0], cost_width=widths[1])
        }
        // TODO: show chained cards in cost column

        // TODO: show played cards
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
