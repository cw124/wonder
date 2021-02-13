//! Allows a human to play 7 Wonders by asking them what action they want to take each turn.

use std::io;
use std::io::Write;

use crate::action::{Action, ActionOptions, Borrowing};
use crate::algorithms::PlayingAlgorithm;
use crate::game::VisibleGame;
use crate::player::Player;
use crate::table::Table;
use itertools::Itertools;

#[derive(Debug)]
pub struct Human;

impl Human {
    /// Prints out the current game state for the given user index.
    fn print_state_for_user(player: &Player, visible_game: &VisibleGame) {
        let all_players = visible_game.public_players;
        let player_index = visible_game.player_index;

        // Offset the players so the player we're controller ends up in the middle.
        let offset = (all_players.len() / 2 + 1) + player_index;
        for i in 0..all_players.len() {
            let index: usize = (i + offset) % all_players.len();
            let other_player = &all_players[index];

            let mut played = Table::new(vec![String::from("Card"), String::from("Power")]);
            other_player
                .built_structures
                .iter()
                .map(|card| vec![card.to_string(), card.power().to_string()])
                .for_each(|row| played.add(row));

            println!(
                "Player {}{}",
                index + 1,
                if index == player_index { " (you)" } else { "" }
            );
            println!(
                "  Wonder: {} (side {:?}). Starting resource: {}",
                other_player.wonder.wonder_type.name(),
                other_player.wonder.wonder_side,
                other_player.wonder.starting_resource()
            );
            println!("  Coins: {}", other_player.coins);
            if !other_player.built_structures.is_empty() {
                played.print("  ", 4);
            }
            println!();
        }

        let mut hand = Table::new(vec![
            String::from("Num"),
            String::from("Card"),
            String::from("Cost"),
            String::from("Power"),
        ]);
        player
            .hand()
            .iter()
            .enumerate()
            .map(|(i, card)| {
                let options = player.options_for_card(card, visible_game);
                let playability = if !options.possible() {
                    "  "
                } else if options.own_cards_only() {
                    "* "
                } else {
                    "# "
                };
                vec![
                    playability.to_string() + &(i + 1).to_string(),
                    card.to_string(),
                    card.cost().to_string(),
                    card.power().to_string(),
                ]
            })
            .for_each(|row| hand.add(row));

        println!("Your hand:");
        hand.print("  ", 4);

        // TODO: show wonder stages and which have been built
        // TODO: show chained cards in cost column
    }

    /// Displays the current state of the game to the user (using [`Human::print_state_for_user`]) and then interactively
    /// asks the user for their action.
    fn ask_for_action(player: &Player, visible_game: &VisibleGame) -> Action {
        // TODO: Support building a wonder stage.
        // TODO: Support borrowing resources from neighbours.

        println!();
        println!();
        Self::print_state_for_user(player, visible_game);

        let hand = player.hand();

        let action = loop {
            println!();
            print!("Please enter the id of the card to play: ");
            let card = *Self::choose_from_slice(&hand);

            print!("And now choose (b) to build or (d) to discard: ");
            let action = 'outer: loop {
                io::stdout().flush().unwrap();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                match choice.trim().to_lowercase().as_str() {
                    "b" => {
                        let options = player.options_for_card(&card, visible_game);
                        if options.own_cards_only() || !options.possible() {
                            // Use own cards, or action not possible (which is caught later).
                            break Action::Build(card, Borrowing::no_borrowing());
                        } else if options.actions.len() == 1 {
                            // Borrowing, but only one option, so just do it.
                            break options.actions[0].clone();
                        } else {
                            // Have user select which borrowing option to go with.
                            println!();
                            println!("Options for borrowing required resources:");
                            Self::print_borrowing_options(
                                &options,
                                visible_game.left_neighbour_index(),
                                visible_game.right_neighbour_index(),
                                &mut io::stdout(),
                            );
                            print!("Please enter the id of the borrow you want to make: ");
                            break 'outer Self::choose_from_slice(&options.actions).clone();
                        }
                    }
                    "d" => break Action::Discard(card),
                    _ => {}
                };
                print!("Please enter either b or d: ");
            };

            if player.can_play(&action, visible_game) {
                break action;
            } else {
                println!("You can't play that card. Please try again");
            }
        };

        println!();
        action
    }

    /// Asks the user to choose one of the items in the given slice.
    fn choose_from_slice<T>(slice: &[T]) -> &T {
        loop {
            io::stdout().flush().unwrap(); // Needed so that print! (with no carriage return) flushes to the terminal.
            let mut id = String::new();
            io::stdin().read_line(&mut id).unwrap();
            let id: usize = id.trim().parse().unwrap_or(0);
            if id > 0 && id <= slice.len() {
                return &slice[id - 1];
            }
            print!("Please enter a number between 1 and {} inclusive: ", slice.len());
        }
    }

    /// Prints the borrowing options the user has available to them.
    fn print_borrowing_options<W: Write>(
        options: &ActionOptions,
        left_neighbour_index: usize,
        right_neighbour_index: usize,
        out: &mut W,
    ) {
        for (index, option) in options.actions.iter().enumerate() {
            if let Action::Build(_, borrowing) = option {
                let mut borrows = vec![];
                if !borrowing.left.is_empty() {
                    borrows.push(format!(
                        "{} from player {}",
                        borrowing.left.iter().map(|borrow| borrow.card).format(", "),
                        left_neighbour_index + 1
                    ));
                }
                if !borrowing.right.is_empty() {
                    borrows.push(format!(
                        "{} from player {}",
                        borrowing.right.iter().map(|borrow| borrow.card).format(", "),
                        right_neighbour_index + 1
                    ));
                }
                writeln!(out, "   {}) Borrow {}", index + 1, borrows.iter().format(" and ")).unwrap();
            }
        }
    }
}

impl PlayingAlgorithm for Human {
    fn get_next_action(&mut self, player: &Player, visible_game: &VisibleGame) -> Action {
        Self::ask_for_action(player, visible_game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Borrow;
    use crate::card::Card;

    #[test]
    fn print_borrowing_options_with_single_borrow() {
        let mut out: Vec<u8> = Vec::new();
        let actions = vec![Action::Build(
            Card::Baths,
            Borrowing::new(vec![Borrow::new(Card::StonePit, 0)], vec![]),
        )];
        Human::print_borrowing_options(&ActionOptions { actions }, 2, 0, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "   1) Borrow Stone Pit from player 3\n"
        );
    }

    #[test]
    fn print_borrowing_options_with_two_borrows_from_same_neighbour() {
        let mut out: Vec<u8> = Vec::new();
        let actions = vec![Action::Build(
            Card::Temple,
            Borrowing::new(
                vec![],
                vec![Borrow::new(Card::LumberYard, 0), Borrow::new(Card::ClayPool, 0)],
            ),
        )];
        Human::print_borrowing_options(&ActionOptions { actions }, 2, 0, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "   1) Borrow Lumber Yard, Clay Pool from player 1\n"
        );
    }

    #[test]
    fn print_borrowing_options_with_one_borrow_from_each_neighbour() {
        let mut out: Vec<u8> = Vec::new();
        let actions = vec![Action::Build(
            Card::Temple,
            Borrowing::new(
                vec![Borrow::new(Card::LumberYard, 0)],
                vec![Borrow::new(Card::ClayPool, 0)],
            ),
        )];
        Human::print_borrowing_options(&ActionOptions { actions }, 2, 0, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "   1) Borrow Lumber Yard from player 3 and Clay Pool from player 1\n"
        );
    }

    #[test]
    fn print_borrowing_options_with_two_options() {
        let mut out: Vec<u8> = Vec::new();
        let actions = vec![
            Action::Build(
                Card::Baths,
                Borrowing::new(vec![Borrow::new(Card::StonePit, 0)], vec![]),
            ),
            Action::Build(
                Card::Baths,
                Borrowing::new(vec![Borrow::new(Card::Excavation, 0)], vec![]),
            ),
        ];
        Human::print_borrowing_options(&ActionOptions { actions }, 2, 0, &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "   1) Borrow Stone Pit from player 3\n   2) Borrow Excavation from player 3\n"
        );
    }
}
