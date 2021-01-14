//! Allows a human to play 7 Wonders by asking them what action they want to take each turn.

use std::io;
use std::io::Write;

use crate::algorithms::PlayingAlgorithm;
use crate::card::Card;
use crate::game::Action;
use crate::player::Player;
use crate::table::Table;

#[derive(Debug)]
pub struct Human {
}

impl Human {
    /// Prints out the current game state for the given user index.
    fn print_state_for_user(player: &Player) {
        let mut hand = Table::new(vec![String::from("Num"), String::from("Card"), String::from("Cost"), String::from("Power")]);
        player.hand().iter().enumerate()
            .map(|(i, card)| vec![(i+1).to_string(), card.to_string(), card.cost().to_string(), card.power().to_string()])
            .for_each(|row| hand.add(row));

        let mut played = Table::new(vec![String::from("Card"), String::from("Power")]);
        player.built_structures().iter()
            .map(|card| vec![card.to_string(), card.power().to_string()])
            .for_each(|row| played.add(row));

        let wonder = player.wonder();
        println!("Wonder: {} (side {:?}). Starting resource: {}",
                 wonder.wonder_type.name(),
                 wonder.wonder_side,
                 wonder.starting_resource());
        println!("Coins: {}", player.coins());
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

    /// Displays the current state of the game to the user (using [`Human::print_state_for_user`]) and then interactively
    /// asks the user for their action.
    fn ask_for_action(player: &Player, player_index: u32) -> Action {
        // TODO: currently this just asks for a card choice, and assumes the card will be "built" (rather than used for
        //  a wonder stage or discarded for coins).
        // TODO: Support borrowing resources from neighbours.
        // TODO: Check the action is actually valid!

        println!();
        println!();
        println!("Player {}, your turn!", player_index + 1);
        println!();
        Self::print_state_for_user(player);

        println!();
        print!("Please enter the id of the card to play: ");
        let hand = player.hand();
        let card: Card = loop {
            io::stdout().flush().unwrap();  // Needed so that print! (with no carriage return) flushes to the terminal.
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let id: usize = match id.trim().parse() {
                Ok(id) => id,
                Err(_) => 0
            };
            if id < 1 || id > hand.len() {
                print!("Please enter a number between 1 and {} inclusive: ", hand.len());
            } else {
                let card = hand[id - 1];
                if !player.can_play(&Action::Build(card)) {
                    print!("You can't play that card. Please try again: ");
                } else {
                    break card;
                }
            }
        };

        let action = Action::Build(card);
        println!("Selected action: {}", action.to_string());

        action
    }
}

impl PlayingAlgorithm for Human {
    fn get_next_action(&self, player: &Player, player_index: u32) -> Action {
        Self::ask_for_action(player, player_index)
    }
}
