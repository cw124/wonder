use std::collections::HashMap;
use std::fmt::Debug;
use std::mem;

use crate::action::{Action, ActionOptions, Borrow, Borrowing};
use crate::card::{Card, Colour};
use crate::game::VisibleGame;
use crate::power::Power;
use crate::power::ScienceItem;
use crate::resources::Resources;
use crate::wonder::{WonderBoard, WonderSide, WonderType};

#[derive(Debug)]
pub struct Player {
    wonder: WonderBoard,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>, // TODO: how to represent this?
    coins: i32,
    hand: Vec<Card>,
}

#[allow(dead_code)]
impl Player {
    pub fn new(wonder_type: WonderType, wonder_side: WonderSide) -> Player {
        Player {
            wonder: WonderBoard {
                wonder_type,
                wonder_side,
            },
            built_structures: vec![],
            built_wonder_stages: vec![],
            coins: 3,
            hand: vec![],
        }
    }

    pub fn wonder(&self) -> &WonderBoard {
        &self.wonder
    }

    pub fn built_structures(&self) -> &Vec<Card> {
        &self.built_structures
    }

    pub fn coins(&self) -> i32 {
        self.coins
    }

    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }

    /// Performs the given [`Action`] on the current player, for example moving a card from the player's hand into the
    /// player's built structures. Returns `true` if the action is legal, `false` otherwise (in which case this function
    /// otherwise does nothing).
    ///
    /// `left_player` and `right_player` are needed in case we need to borrow resources from them, in which case, they
    /// will have the cost credited to them.
    pub fn do_action(
        &mut self,
        action: &Action,
        visible_game: &VisibleGame,
        left_player: &mut Player,
        right_player: &mut Player,
        discard_pile: &mut Vec<Card>,
    ) -> bool {
        // Removes and returns the given card from the player's hand.
        fn remove_from_hand(hand: &mut Vec<Card>, card: &Card) -> Card {
            let index = hand.iter().position(|c| c == card).unwrap();
            hand.swap_remove(index)
        }

        if self.can_play(action, visible_game) {
            match action {
                Action::Build(card, borrowing) => {
                    let card_from_hand = remove_from_hand(&mut self.hand, card);
                    self.built_structures.push(card_from_hand);
                    self.coins -= card_from_hand.cost().coins;
                    // TODO: cost of borrowing needs to vary depending on yellow cards.
                    self.coins -= borrowing.left.len() as i32 * 2 + borrowing.right.len() as i32 * 2;
                    left_player.add_coins(borrowing.left.len() as i32 * 2);
                    right_player.add_coins(borrowing.right.len() as i32 * 2);
                }
                Action::Wonder(_, _) => todo!(),
                Action::Discard(card) => {
                    discard_pile.push(remove_from_hand(&mut self.hand, card));
                    self.coins += 3;
                }
            }
            true
        } else {
            false
        }
    }

    /// Replaces this player's hand with the given cards, returning the hand the player had before the swap.
    pub fn swap_hand(&mut self, new_hand: Vec<Card>) -> Vec<Card> {
        mem::replace(&mut self.hand, new_hand)
    }

    /// Adds the given coins to this player's total.
    fn add_coins(&mut self, coins: i32) {
        self.coins += coins;
    }

    fn evaluate_green(colour_cards: &[Card]) -> f32 {
        let mut science_items_count: HashMap<ScienceItem, i32> = HashMap::new();

        science_items_count.insert(ScienceItem::Compass, 0);
        science_items_count.insert(ScienceItem::Cog, 0);
        science_items_count.insert(ScienceItem::Tablet, 0);

        for card in colour_cards.iter() {
            if let Power::Science(science_items) = card.power() {
                for science_item in science_items.iter() {
                    let count = science_items_count.entry(*science_item).or_insert(0);
                    *count += 1;
                }
            }
        }

        let score_for_sets_of_identical_symbols: f32 = science_items_count
            .iter()
            .filter(|(_, count)| **count > 0)
            .map(|(_, count)| (*count as f32).powf(2f32))
            .sum();

        let score_for_all_symbol_groups: f32 =
            7f32 * *science_items_count.iter().min_by_key(|(_, count)| *count).unwrap().1 as f32;

        score_for_all_symbol_groups + score_for_sets_of_identical_symbols
    }

    fn evaluate_colour(cards_of_given_colour: &[Card]) -> f32 {
        let colour = cards_of_given_colour.get(0).unwrap().colour();

        match colour {
            Colour::Green => Self::evaluate_green(cards_of_given_colour),
            _ => cards_of_given_colour.iter().map(|card| card.immediate_strength()).sum(),
        }
    }

    fn strength_internal(cards: &[Card]) -> f32 {
        let mut colour_to_structure = HashMap::new();
        for structure in cards.iter() {
            let colour_structures = colour_to_structure.entry(structure.colour()).or_insert_with(Vec::new);
            colour_structures.push(*structure)
        }

        colour_to_structure
            .iter()
            .map(|colour_entry| Self::evaluate_colour(colour_entry.1))
            .sum()
    }

    /// Returns this player's "strength" -- a number where a higher value means the player is doing better than a lower
    /// value.
    pub fn strength(&self) -> f32 {
        Self::strength_internal(&self.built_structures)
    }

    pub fn can_play(&self, action: &Action, visible_game: &VisibleGame) -> bool {
        match action {
            Action::Build(card, borrowing) => self.can_play_card(card, borrowing, visible_game),
            Action::Wonder(_, _) => todo!(),
            Action::Discard(card) => self.hand.iter().any(|c| c == card),
        }
    }

    /// Returns `true` if the user can afford to play the given card, given the resources the player
    /// has access to.
    fn can_play_card(&self, card: &Card, borrowing: &Borrowing, visible_game: &VisibleGame) -> bool {
        // Can't play if the player doesn't have the card in hand, or the neighbours haven't built the cards being
        // borrowed.
        if !self.hand.iter().any(|c| c == card) || !borrowing.valid(visible_game) {
            return false;
        }

        // Can play if the borrowing being proposed (possibly no borrowing) is one of the options available, given the
        // neighbour's resources and the player's coins.
        //
        // TODO: it's a bit inefficient to calculate this again. Perhaps an action should exactly define what's
        //  happening so we can directly check it.
        for action in self.options_for_card(card, visible_game).actions {
            if let Action::Build(_, borrowing1) = action {
                if borrowing1 == *borrowing {
                    return true;
                }
            }
        }

        false
    }

    /// Given a card and a [`VisibleGame`], returns an [`ActionOptions`] containing all possible actions that can be
    /// taken to build the card. However, because we do not (currently at least) include in the action which own (ie.
    /// non borrowed) cards are used, only a single action is ever returned where there is no borrowing. In other words,
    /// if the player can afford a card using several different combinations of their own built structures and/or
    /// starting wonder resource, then only a single action will represent all of these combinations. If the user has
    /// several different options when borrowing, each of these is returned as a separate action. This allows the player
    /// to choose how much money to spend on borrowing, and how much to give to each neighbour.
    ///
    /// If the player cannot play the card, an empty vector is returned.
    ///
    /// Note this function doesn't verify the cards the player has in their hand, meaning `card` can be a card the
    /// player doesn't have. As long as they can afford it, valid actions will be returned to achieve it.
    pub fn options_for_card(&self, card: &Card, visible_game: &VisibleGame) -> ActionOptions {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum Source {
            Own,
            LeftNeighbour,
            RightNeighbour,
        }

        struct UsableResources {
            card: Card,
            index: u32,
            resources: Vec<Resources>,
            source: Source,
        }

        /// Given some own cards or a neighbour's cards, adds to `choices` the things we need to consider in order to
        /// find all possible ways of achieving the required resources. Cards that provide only resources we don't need
        /// are removed entirely. Cards that provide options of resources are reduced to only those resources we
        /// require.
        fn add_choices(
            cards: &[Card],
            required_resources: &Resources,
            source: Source,
            choices: &mut Vec<UsableResources>,
        ) {
            for card in cards {
                match (card.power(), source) {
                    // Make sure we only borrow brown and grey cards from neighbours (not yellow).
                    (Power::Producer(resource_options), Source::Own)
                    | (Power::PurchasableProducer(resource_options), _) => {
                        // Filter out single choice own cards as we'll have already dealt with these.
                        if source != Source::Own || resource_options.len() > 1 {
                            let resources: Vec<Resources> = resource_options
                                .iter()
                                .filter(|r| r.has(required_resources))
                                .cloned()
                                .collect();
                            // This is a bit hacky, hopefully improve this one day: if this is a "double" resource card
                            // (eg. sawmill), add two UsableResources with 1 resource each (so we can choose to use only
                            // one resource or both).
                            if resources.len() == 1 && resources[0].max() == 2 {
                                for i in 0..2 {
                                    choices.push(UsableResources {
                                        card: *card,
                                        index: i as u32,
                                        resources: vec![resources[0].split()],
                                        source,
                                    });
                                }
                            } else if !resources.is_empty() {
                                choices.push(UsableResources {
                                    card: *card,
                                    index: 0,
                                    resources,
                                    source,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Get the cost of the card, and subtract the Wonder starting resources and any non-choice resources owned by
        // the player.
        let mut required_resources = card.cost();
        required_resources -= &self.wonder.starting_resource();
        required_resources.coins -= self.coins;
        for card in &self.built_structures {
            if let Power::Producer(resources) | Power::PurchasableProducer(resources) = card.power() {
                if resources.len() == 1 {
                    required_resources -= &resources[0];
                }
            }
        }
        if required_resources.satisfied() {
            // Can afford with own resources.
            return ActionOptions {
                actions: vec![Action::Build(*card, Borrowing::no_borrowing())],
            };
        }

        // We now add all choice cards owned by the player, and all borrowable resources owned by their neighbours, and
        // iterate over all possible combinations of those cards. We filter our entire cards that don't have the
        // resources we need, and filter choice cards to just the resources required.
        let mut choices = vec![];
        add_choices(&self.built_structures, &required_resources, Source::Own, &mut choices);
        add_choices(
            &visible_game.left_neighbour().built_structures,
            &required_resources,
            Source::LeftNeighbour,
            &mut choices,
        );
        add_choices(
            &visible_game.right_neighbour().built_structures,
            &required_resources,
            Source::RightNeighbour,
            &mut choices,
        );

        let mut actions = vec![];
        if !choices.is_empty() {
            let mut combinations = 1;
            for choice in &choices {
                combinations *= if choice.source == Source::Own {
                    // We always choose our own resources, because there's no cost to doing so and we must use them
                    // rather than borrow if possible.
                    choice.resources.len() as u32
                } else {
                    // For neighbours, we can choose each possible option as well as not choosing the card at all.
                    choice.resources.len() as u32 + 1
                }
            }

            'outer: for combination in 0..combinations {
                let mut r = required_resources.clone();
                let mut c = combination;
                let mut left_borrowing = vec![];
                let mut right_borrowing = vec![];
                for choice in &choices {
                    // If own resource, always include it. If neighbours', also try without it.
                    let len = choice.resources.len() + if choice.source == Source::Own { 0 } else { 1 };
                    let index = (c % len as u32) as usize;
                    if choice.source == Source::Own {
                        r -= &choice.resources[index];
                    } else if index > 0 {
                        // TODO: cost of borrowing needs to vary depending on yellow cards.
                        if r.coins <= -2 {
                            if r.not_needed(&choice.resources[index - 1]) {
                                // We already have enough of whatever this option provides. Therefore, this particular
                                // combination is not valid. Skip to the next.
                                continue 'outer;
                            }
                            r -= &choice.resources[index - 1];
                            r.coins += 2;
                            if choice.source == Source::LeftNeighbour {
                                left_borrowing.push(Borrow::new(choice.card, choice.index));
                            } else {
                                right_borrowing.push(Borrow::new(choice.card, choice.index));
                            }
                        } else {
                            // Out of money for borrowing.
                            continue 'outer;
                        }
                    }
                    c /= len as u32;
                }
                if r.satisfied() {
                    actions.push(Action::Build(*card, Borrowing::new(left_borrowing, right_borrowing)));
                }
            }
        }

        ActionOptions { actions }
    }
}

/// Represents the aspects of [`Player`] that are public knowledge (ie. visible on the table). Things like a player's
/// current hand are not included.
#[derive(Debug)]
pub struct PublicPlayer {
    pub wonder: WonderBoard,
    pub built_structures: Vec<Card>,
    pub coins: i32,
}

impl PublicPlayer {
    /// Creates a [`PublicPlayer`] from a [`Player`], copy/cloning the values so the originals can be mutated later
    /// without issue.
    pub fn new(player: &Player) -> PublicPlayer {
        PublicPlayer {
            wonder: player.wonder,
            built_structures: player.built_structures.clone(),
            coins: player.coins,
        }
    }
}

#[cfg(test)]
mod tests {
    use Card::*;

    use super::*;

    #[test]
    fn options_for_card_returns_nothing_if_insufficient_resources() {
        // Stockade requires 1 wood, we have 1 ore (starting resource).
        let player = new_player(vec![]);
        assert_eq!(
            0,
            player
                .options_for_card(&Stockade, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_nothing_if_insufficient_coins() {
        // Tree farm requires 1 coin, which we don't have.
        let mut player = new_player(vec![]);
        player.coins = 0;
        assert_eq!(
            0,
            player
                .options_for_card(&TreeFarm, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_one_option_if_sufficient_resources() {
        // Barracks requires 1 ore, which we have (starting resource).
        let player = new_player(vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&Barracks, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_one_option_if_sufficient_coins() {
        // Tree farm requires 1 coin, which we have (starting coins).
        let player = new_player(vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&TreeFarm, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_one_option_if_free() {
        // Lumber yard is free.
        let player = new_player(vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&LumberYard, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_handles_option_cards() {
        // Stockade requires 1 wood. Tree farm provides wood or clay.
        let mut player = new_player(vec![TreeFarm]);
        build(&mut player, TreeFarm);
        assert_eq!(
            1,
            player
                .options_for_card(&Stockade, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_nothing_if_both_options_needed() {
        // Temple requires 1 wood, 1 clay, 1 glass. Tree farm provides wood or clay, which is no good.
        let mut player = new_player(vec![TreeFarm, Glassworks1]);
        build(&mut player, TreeFarm);
        build(&mut player, Glassworks1);
        assert_eq!(
            0,
            player
                .options_for_card(&Temple, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_one_option_even_if_multiple_ways() {
        // Stockade requires 1 wood. Both lumber yard and tree farm provide it.
        let mut player = new_player(vec![LumberYard, TreeFarm]);
        build(&mut player, LumberYard);
        build(&mut player, TreeFarm);
        assert_eq!(
            1,
            player
                .options_for_card(&Stockade, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_handles_multiple_resources_required() {
        // Caravansery requires 2 wood. Lumber yard and tree farm provide one each.
        let mut player = new_player(vec![LumberYard, TreeFarm]);
        build(&mut player, LumberYard);
        build(&mut player, TreeFarm);
        assert_eq!(
            1,
            player
                .options_for_card(&Caravansery, &visible_game(&players()))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_borrows_from_neighbours() {
        // Stockade requires 1 wood, we can borrow from a neighbour.
        let player = new_player(vec![]);
        let public_players = players_with_resources(vec![LumberYard], vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&Stockade, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_borrow_if_insufficient_coins() {
        // Stockade requires 1 wood, we can borrow from a neighbour, but we don't have enough coins.
        let mut player = new_player(vec![]);
        player.coins = 1;
        let public_players = players_with_resources(vec![LumberYard], vec![]);
        assert_eq!(
            0,
            player
                .options_for_card(&Stockade, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_borrow_if_insufficient_coins2() {
        // Caravansery requires 2 wood, we can borrow from a neighbour, but we don't have enough coins.
        let mut player = new_player(vec![]);
        player.coins = 3;
        let public_players = players_with_resources(vec![LumberYard, TreeFarm], vec![]);
        assert_eq!(
            0,
            player
                .options_for_card(&Caravansery, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_borrow_if_insufficient_coins3() {
        // Caravansery requires 2 wood, we can borrow from a neighbour, but we don't have enough coins.
        let mut player = new_player(vec![]);
        player.coins = 3;
        let public_players = players_with_resources(vec![Sawmill], vec![]);
        assert_eq!(
            0,
            player
                .options_for_card(&Caravansery, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_two_options_if_both_neighbours_can_be_borrowed_from() {
        // Stockade requires 1 wood, we can borrow from either neighbour.
        let player = new_player(vec![]);
        let public_players = players_with_resources(vec![LumberYard], vec![TreeFarm]);
        assert_eq!(
            2,
            player
                .options_for_card(&Stockade, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_borrows_from_both_neighbours() {
        // Caravansery requires 2 wood. Lumber yard and tree farm, one from each neighbour, provide one each.
        let mut player = new_player(vec![]);
        player.coins = 4;
        let public_players = players_with_resources(vec![LumberYard], vec![TreeFarm]);
        assert_eq!(
            1,
            player
                .options_for_card(&Caravansery, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_borrow_unneeded_resources() {
        // Stockade requires 1 wood. While we could borrow the wood from the neighbour's lumber yard, we have one of our
        // own which we should use.
        let mut player = new_player(vec![TreeFarm]);
        build(&mut player, TreeFarm);
        let public_players = players_with_resources(vec![LumberYard], vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&Stockade, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_borrow_unneeded_resources2() {
        // Stockade requires 1 wood, we can borrow from a neighbour, the options shouldn't include also borrowing the
        // stone pit, even though we can afford to do that because it's not needed.
        let mut player = new_player(vec![]);
        player.coins = 4;
        let public_players = players_with_resources(vec![LumberYard, StonePit], vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&Stockade, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_borrows_fractional_resources() {
        // Laboratory requires 2 clay (and 1 papyrus, but don't worry about that). Valid borrows are brickyard only, or
        // clay pit and clay pool, or brickyard and clay pit/clay pool, even though the last option uses only 1 of the 2
        // clay provided by the brickyard in each case. Note that 6 combinations are generated because borrowing, for
        // example, clay pool and the first clay of the brickyard is counted as a distinct option to borrowing clay pool
        // and the second clay of the brickyard. Maybe we want to change this in future.
        let mut player = new_player(vec![Press1]);
        build(&mut player, Press1);
        player.coins = 4;
        let public_players = players_with_resources(vec![ClayPit, Brickyard], vec![ClayPool]);
        assert_eq!(
            6,
            player
                .options_for_card(&Laboratory, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_cause_duplicate_options_due_to_unneeded_own_resources() {
        // Aqueduct requires 3 stone, which must come from our own stone pit and the neighbour's quarry. Our own clay
        // pit should not affect anything (a bug caused it to duplicate the only valid option due to iterating over the
        // clay pit options).
        let mut player = new_player(vec![StonePit, ClayPit]);
        build(&mut player, StonePit);
        build(&mut player, ClayPit);
        player.coins = 4;
        let public_players = players_with_resources(vec![Quarry], vec![]);
        assert_eq!(
            1,
            player
                .options_for_card(&Aqueduct, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_does_not_borrow_yellow_cards() {
        // Stockade requires 1 wood. Our neighbour has a caravansery, which produces wood, but it is not borrowable.
        let player = new_player(vec![]);
        let public_players = players_with_resources(vec![Caravansery], vec![]);
        assert_eq!(
            0,
            player
                .options_for_card(&Stockade, &visible_game(&public_players))
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_uses_own_yellow_cards() {
        // Baths requires 1 stone. Caravansery produces 1 stone. It's yellow, but we own it, so we can use it.
        let mut player = new_player(vec![Sawmill, Caravansery]);
        build(&mut player, Sawmill);
        build(&mut player, Caravansery);
        assert_eq!(
            1,
            player.options_for_card(&Baths, &visible_game(&players())).actions.len()
        );
    }

    #[test]
    fn can_play_returns_false_if_player_does_not_have_card() {
        let player = new_player(vec![LumberYard]);
        assert_eq!(
            false,
            player.can_play(
                &Action::Build(StonePit, Borrowing::no_borrowing()),
                &visible_game(&players())
            )
        );
    }

    #[test]
    fn can_play_returns_false_if_player_does_not_have_card2() {
        let player = new_player(vec![LumberYard]);
        assert_eq!(
            false,
            player.can_play(&Action::Discard(StonePit), &visible_game(&players()))
        );
    }

    #[test]
    fn can_play_returns_true_if_player_does_have_card_and_card_is_playable() {
        let player = new_player(vec![LumberYard]);
        assert_eq!(
            true,
            player.can_play(
                &Action::Build(LumberYard, Borrowing::no_borrowing()),
                &visible_game(&players())
            )
        );
    }

    #[test]
    fn can_play_returns_false_if_borrowing_not_possible() {
        // Stockade requires 1 wood, we *can* borrow from a neighbour, but our action says we're not doing any borrowing.
        let player = new_player(vec![Stockade]);
        let public_players = players_with_resources(vec![LumberYard], vec![]);
        assert_eq!(
            false,
            player.can_play(
                &Action::Build(Stockade, Borrowing::no_borrowing()),
                &visible_game(&public_players)
            )
        );
    }

    #[test]
    fn strength_returns_sum_of_card_strengths() {
        assert_eq!(0.0, Player::strength_internal(&[StonePit]));
        assert_eq!(5.0, Player::strength_internal(&[StonePit, Quarry, Aqueduct]));
        assert_eq!(
            6.0,
            Player::strength_internal(&[StonePit, Quarry, Aqueduct, Loom1, Apothecary])
        );
    }

    #[test]
    fn strength_returns_correct_strength_of_green_structures() {
        assert_eq!(1.0, Player::strength_internal(&[Lodge]));
        assert_eq!(4.0, Player::strength_internal(&[Lodge, Apothecary]));
        assert_eq!(9.0, Player::strength_internal(&[Lodge, Apothecary, Dispensary]));
        assert_eq!(10.0, Player::strength_internal(&[Lodge, Workshop, Library]));
        assert_eq!(
            21.0,
            Player::strength_internal(&[Lodge, Apothecary, Dispensary, Laboratory, Workshop, Library])
        ); // rulebook example
    }

    #[test]
    fn do_action_returns_false_if_action_not_playable() {
        let mut player = new_player(vec![LumberYard]);
        assert_eq!(false, build(&mut player, StonePit));
    }

    #[test]
    fn do_action_transfers_built_card_from_hand_to_built_structures() {
        let mut player = new_player(vec![LumberYard]);
        assert_eq!(0, player.built_structures.len());
        assert_eq!(1, player.hand.len());
        assert_eq!(true, build(&mut player, LumberYard));
        assert_eq!(1, player.built_structures.len());
        assert_eq!(0, player.hand.len());
    }

    #[test]
    fn do_action_decrements_cost_in_coins_when_building() {
        let mut player = new_player(vec![TreeFarm]);
        assert_eq!(3, player.coins);
        assert_eq!(true, build(&mut player, TreeFarm));
        assert_eq!(2, player.coins);
    }

    #[test]
    fn do_action_transfers_discarded_card_from_hand_to_discard_pile() {
        let mut player = new_player(vec![LumberYard]);
        let mut discard_pile = vec![];
        assert_eq!(1, player.hand.len());
        assert_eq!(
            true,
            player.do_action(
                &Action::Discard(LumberYard),
                &visible_game(&players()),
                &mut new_player(vec![]),
                &mut new_player(vec![]),
                &mut discard_pile
            )
        );
        assert_eq!(1, discard_pile.len());
        assert_eq!(0, player.hand.len());
    }

    #[test]
    fn do_action_adds_three_coins_when_discarding() {
        let mut player = new_player(vec![LumberYard]);
        assert_eq!(3, player.coins);
        assert_eq!(
            true,
            player.do_action(
                &Action::Discard(LumberYard),
                &visible_game(&players()),
                &mut new_player(vec![]),
                &mut new_player(vec![]),
                &mut vec![]
            )
        );
        assert_eq!(6, player.coins);
    }

    #[test]
    fn new_public_player() {
        let player = new_player(vec![LumberYard]);
        let public_player = PublicPlayer::new(&player);
        assert_eq!(player.wonder, public_player.wonder);
        assert_eq!(player.built_structures, public_player.built_structures);
        assert_eq!(player.coins, public_player.coins);
    }

    fn new_player(hand: Vec<Card>) -> Player {
        let mut player = Player::new(WonderType::ColossusOfRhodes, WonderSide::A);
        player.swap_hand(hand);
        player
    }

    fn visible_game(public_players: &[PublicPlayer]) -> VisibleGame {
        VisibleGame {
            public_players,
            player_index: 1,
        }
    }

    fn players() -> Vec<PublicPlayer> {
        players_with_resources(vec![], vec![])
    }

    fn players_with_resources(left: Vec<Card>, right: Vec<Card>) -> Vec<PublicPlayer> {
        vec![
            PublicPlayer {
                wonder: WonderBoard {
                    wonder_type: WonderType::ColossusOfRhodes,
                    wonder_side: WonderSide::A,
                },
                built_structures: right,
                coins: 0,
            },
            PublicPlayer {
                wonder: WonderBoard {
                    wonder_type: WonderType::LighthouseOfAlexandria,
                    wonder_side: WonderSide::A,
                },
                built_structures: vec![],
                coins: 0,
            },
            PublicPlayer {
                wonder: WonderBoard {
                    wonder_type: WonderType::TempleOfArtemis,
                    wonder_side: WonderSide::A,
                },
                built_structures: left,
                coins: 0,
            },
        ]
    }

    fn build(player: &mut Player, card: Card) -> bool {
        let mut left_neighbour = new_player(vec![]);
        let mut right_neighbour = new_player(vec![]);
        player.do_action(
            &Action::Build(card, Borrowing::no_borrowing()),
            &visible_game(&players()),
            &mut left_neighbour,
            &mut right_neighbour,
            &mut vec![],
        )
    }
}
