//! Represents the whole game state.

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use crate::card;
use crate::card::{Age, Card};
use crate::player::{Player, PublicPlayer};
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

        // For each player, pick a random wonder and deal seven random cards.
        let players: Vec<Player> = algorithms.into_iter()
            .zip(wonder_types)
            .map(|(algorithm, wonder_type)| Player::new(wonder_type, WonderSide::A, algorithm))
            .collect();

        Game {
            players,
            turn: 0,
            discard_pile: vec![]
        }
    }

    /// Executes a turn of the game. Gets an [`Action`] from each [`Player`] and updates the game state accordingly.
    pub fn do_turn(&mut self) {
        // At the start of each age, deal new cards and add any remaining cards to the discard pile.
        if self.turn % 6 == 0 {
            let mut deck = card::new_deck(self.age(), self.player_count());
            for player in self.players.iter_mut() {
                let old_hand = player.swap_hand(deck.drain(0..7).collect());
                for card in old_hand {
                    self.discard_pile.push(card);
                }
            }
        }

        // Do actions. public_players is an immutable snapshot of the game state before players start moving, so
        // that each moves "simultaneously".
        let public_players: Vec<PublicPlayer> = self.players.iter()
            .map(|player| PublicPlayer::new(&player))
            .collect();
        for index in 0..self.players.len() {
            let (mut right_player, player, mut left_player) =
                Self::get_mutable_player_and_neighbours(&mut self.players, index);
            let visible_game = VisibleGame { players: &public_players, player_index: index };
            let action = player.algorithm().get_next_action(&player, &visible_game);
            player.do_action(&action, &visible_game, &mut left_player, &mut right_player, &mut self.discard_pile);
        }

        // Pass cards.
        let num_players = self.players.len();
        let mut hand = vec![];
        for i in 0..num_players + 1 {
            let index = if self.age() == Age::Second {
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

    pub fn player_count(&self) -> u32 {
        self.players.len() as u32
    }

    /// Returns the current age being played.
    pub fn age(&self) -> Age {
        match self.turn {
            0..=5 => Age::First,
            6..=11 => Age::Second,
            12..=17 => Age::Third,
            _ => panic!("Unknown turn!")
        }
    }

    /// Given the index of a player, returns a mutable borrow of that player, as well as the left and right neighbours
    /// of the player. This is super-horrible in Rust as far as I can tell. Perhaps there's a better way...
    fn get_mutable_player_and_neighbours(players: &mut Vec<Player>, index: usize)
            -> (&mut Player, &mut Player, &mut Player) {
        if index == 0 {
            // player=0, left=1, right=n
            let (player, after) = players.split_first_mut().unwrap();
            let (right_player, rest) = after.split_last_mut().unwrap();
            let (left_player, _) = rest.split_first_mut().unwrap();
            (right_player, player, left_player)

        } else if index == players.len() - 1 {
            // player=n, left=0, right=n-1
            let (player, before) = players.split_last_mut().unwrap();
            let (left_player, rest) = before.split_first_mut().unwrap();
            let (right_player, _) = rest.split_last_mut().unwrap();
            (right_player, player, left_player)

        } else {
            // player=i, left=i+1, right=i-1
            let (before, player_and_after) = players.split_at_mut(index);
            let (player_slice, after) = player_and_after.split_at_mut(1);
            (&mut before[index - 1], &mut player_slice[0], &mut after[0])
        }
    }
}

/// The state of the game visible to all players (ie. excluding things like players' hands).
#[derive(Debug)]
pub struct VisibleGame<'a> {
    /// All players in the game.
    pub players: &'a [PublicPlayer],
    /// The index of the player this has been generated for.
    pub player_index: usize,
}

impl<'a> VisibleGame<'a> {
    /// Returns the [`PublicPlayer`] on the current player's left, ie. clockwise.
    pub fn left_neighbour(&self) -> &PublicPlayer {
        &self.players[self.left_neighbour_index()]
    }

    /// Returns the [`PublicPlayer`] on the current player's right, ie. anti-clockwise.
    pub fn right_neighbour(&self) -> &PublicPlayer {
        &self.players[self.right_neighbour_index()]
    }

    /// Returns the 0-based index of the left neighbour.
    pub fn left_neighbour_index(&self) -> usize {
        (self.player_index + 1) % self.players.len()
    }

    /// Returns the 0-based index of the right neighbour.
    pub fn right_neighbour_index(&self) -> usize {
        (self.player_index + self.players.len() - 1) % self.players.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::random::Random;
    use crate::action::Action;

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
        let mut game = Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]);
        assert_eq!(Age::First, game.age());
        for _i in 0..7 {
            game.do_turn();
        }
        assert_eq!(Age::Second, game.age());
        for _i in 0..7 {
            game.do_turn();
        }
        assert_eq!(Age::Third, game.age());
    }

    #[test]
    fn do_turn_deals_new_cards_at_the_start_of_each_age() {
        let mut game = Game::new(vec![Box::new(Random {}), Box::new(Random {}), Box::new(Random {})]);
        game.do_turn();
        assert_eq!(6, game.players[0].hand().len());
        for _i in 0..6 {
            game.do_turn();
        }
        assert_eq!(6, game.players[0].hand().len());
        for _i in 0..6 {
            game.do_turn();
        }
        assert_eq!(6, game.players[0].hand().len());
    }

    #[test]
    fn do_turn_rotates_hands() {
        let mut game = Game::new(vec![
            Box::new(AlwaysDiscards {}),
            Box::new(AlwaysDiscards {}),
            Box::new(AlwaysDiscards {})]);

        // We have to do an initial turn so the first age cards are dealt to the players. Before this, nobody has any
        // cards!
        game.do_turn();

        let player0 = game.players[0].hand().clone();
        let player1 = game.players[1].hand().clone();
        let player2 = game.players[2].hand().clone();

        game.do_turn();

        assert_eq!(game.players[1].hand()[..], player0[..player0.len()-1]);
        assert_eq!(game.players[2].hand()[..], player1[..player0.len()-1]);
        assert_eq!(game.players[0].hand()[..], player2[..player0.len()-1]);
    }

    #[test]
    fn get_mutable_player_and_neighbours() {
        let mut players = vec![
            Player::new(WonderType::ColossusOfRhodes, WonderSide::A, Box::new(Random {})),
            Player::new(WonderType::LighthouseOfAlexandria, WonderSide::A, Box::new(Random {})),
            Player::new(WonderType::TempleOfArtemis, WonderSide::A, Box::new(Random {})),
            Player::new(WonderType::HangingGardensOfBabylon, WonderSide::A, Box::new(Random {})),
        ];

        let (right, player, left) = Game::get_mutable_player_and_neighbours(&mut players, 0);
        assert_eq!(WonderType::HangingGardensOfBabylon, right.wonder().wonder_type);
        assert_eq!(WonderType::ColossusOfRhodes, player.wonder().wonder_type);
        assert_eq!(WonderType::LighthouseOfAlexandria, left.wonder().wonder_type);

        let (right, player, left) = Game::get_mutable_player_and_neighbours(&mut players, 1);
        assert_eq!(WonderType::ColossusOfRhodes, right.wonder().wonder_type);
        assert_eq!(WonderType::LighthouseOfAlexandria, player.wonder().wonder_type);
        assert_eq!(WonderType::TempleOfArtemis, left.wonder().wonder_type);

        let (right, player, left) = Game::get_mutable_player_and_neighbours(&mut players, 3);
        assert_eq!(WonderType::TempleOfArtemis, right.wonder().wonder_type);
        assert_eq!(WonderType::HangingGardensOfBabylon, player.wonder().wonder_type);
        assert_eq!(WonderType::ColossusOfRhodes, left.wonder().wonder_type);
    }

    /// Always discards the last card in the hand.
    #[derive(Debug)]
    pub struct AlwaysDiscards;
    impl PlayingAlgorithm for AlwaysDiscards {
        fn get_next_action(&self, player: &Player, _visible_game: &VisibleGame) -> Action {
            // TODO: we always discard the last card so the order of the hand is not disrupted (because
            //  player::do_action uses Vec::swap_remove). Ideally don't rely on the implementation of do_action. But
            //  that involves sorting the hands in order to compare them, which is painful (at least with my current
            //  Rust skills).
            Action::Discard(player.hand()[player.hand().len()-1])
        }
    }
}
