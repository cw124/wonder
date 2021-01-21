//! Allows a human to play 7 Wonders by asking them what action they want to take each turn.

use std::io;
use std::io::Write;

use crate::algorithms::PlayingAlgorithm;
use crate::game::Action;
use crate::player::{Player, PublicPlayer};
use crate::table::Table;

#[derive(Debug)]
pub struct Human;

impl Human {
    /// Prints out the current game state for the given user index.
    fn print_state_for_user(player: &Player, player_index: u32, all_players: &[PublicPlayer]) {
        // Offset the players so the player we're controller ends up in the middle.
        let offset = (all_players.len() / 2 + 1) + player_index as usize;
        for i in 0..all_players.len() {
            let index: usize = (i + offset) % all_players.len();
            let other_player = &all_players[index];

            let mut played = Table::new(vec![String::from("Card"), String::from("Power")]);
            other_player.built_structures.iter()
                .map(|card| vec![card.to_string(), card.power().to_string()])
                .for_each(|row| played.add(row));

            println!("Player {}{}", index + 1, if index as u32 == player_index { " (you)" } else { "" });
            println!("  Wonder: {} (side {:?}). Starting resource: {}",
                     other_player.wonder.wonder_type.name(),
                     other_player.wonder.wonder_side,
                     other_player.wonder.starting_resource());
            println!("  Coins: {}", other_player.coins);
            if !other_player.built_structures.is_empty() {
                played.print("  ", 4);
            }
            println!();
        }

        let mut hand = Table::new(vec![String::from("Num"), String::from("Card"), String::from("Cost"), String::from("Power")]);
        player.hand().iter().enumerate()
            .map(|(i, card)| vec![(i + 1).to_string(), card.to_string(), card.cost().to_string(), card.power().to_string()])
            .for_each(|row| hand.add(row));

        println!("Your hand:");
        hand.print("  ", 4);

        // TODO: show wonder stages and which have been built
        // TODO: show chained cards in cost column
    }

    /// Displays the current state of the game to the user (using [`Human::print_state_for_user`]) and then interactively
    /// asks the user for their action.
    fn ask_for_action(player: &Player, player_index: u32, all_players: &[PublicPlayer]) -> Action {
        // TODO: Support building a wonder stage.
        // TODO: Support borrowing resources from neighbours.

        println!();
        println!();
        Self::print_state_for_user(player, player_index, all_players);

        let hand = player.hand();

        let action = loop {
            println!();
            print!("Please enter the id of the card to play: ");

            let card = loop {
                io::stdout().flush().unwrap();  // Needed so that print! (with no carriage return) flushes to the terminal.
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: usize = match id.trim().parse() {
                    Ok(id) => id,
                    Err(_) => 0
                };
                if id > 0 && id <= hand.len() {
                    break hand[id - 1];
                }
                print!("Please enter a number between 1 and {} inclusive: ", hand.len());
            };

            print!("And now choose (b) to build or (d) to discard: ");
            let action = loop {
                io::stdout().flush().unwrap();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                match choice.trim().to_lowercase().as_str() {
                    "b" => break Action::Build(card),
                    "d" => break Action::Discard(card),
                    _ => {},
                };
                print!("Please enter either b or d: ");
            };

            if player.can_play(&action) {
                break action;
            } else {
                println!("You can't play that card. Please try again");
            }
        };

        println!("Selected action: {}", action.to_string());

        action
    }
}

impl PlayingAlgorithm for Human {
    fn get_next_action(&self, player: &Player, player_index: u32, all_players: &[PublicPlayer]) -> Action {
        Self::ask_for_action(player, player_index, all_players)
    }
}
