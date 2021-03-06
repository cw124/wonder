use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::Debug;
use std::mem;

use crate::action::{Action, ActionOptions, Borrow, Borrowing};
use crate::card::{Card, Colour};
use crate::game::VisibleGame;
use crate::power::ScienceItem;
use crate::power::{Power, ProducedResources};
use crate::resources::{Cost, Resource};
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

    /// Creates a new player from a public player. Intended for playing algorithms who need to simulate a game.
    pub fn new_from_public(public_player: &PublicPlayer, hand: Vec<Card>) -> Player {
        Player {
            wonder: public_player.wonder,
            built_structures: public_player.built_structures.clone(),
            built_wonder_stages: vec![],
            coins: public_player.coins,
            hand,
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
        /// Checks the given borrows against the given player, making sure the player has the right cards available.
        /// The resources provided by the borrows are subtracted from `cost`, and the coins needed for the borrows are
        /// added to `cost`.
        fn check(borrows: &[Borrow], public_player: &PublicPlayer, cost: &mut Cost) -> bool {
            let mut choices = vec![];
            add_choices(
                &public_player.built_structures,
                &cost,
                Source::LeftNeighbour, // Doesn't really matter as long as not Source::Own
                &mut choices,
            );
            for borrow in borrows {
                // Find and remove a card that matches. If we can't find one, the borrow is illegal.
                let choice = choices
                    .iter()
                    .position(|usable| usable.card == borrow.card && usable.resources.contains(&borrow.resource))
                    .map(|index| choices.swap_remove(index));
                match choice {
                    Some(_) => {
                        *cost -= &borrow.resource;
                        cost.coins += 2; // TODO: cost of borrowing needs to vary depending on yellow cards.
                    }
                    None => return false,
                }
            }
            true
        }

        // Can't play if the player doesn't have the card in hand.
        if !self.hand.iter().any(|c| c == card) {
            return false;
        }

        // Reduce the cost of the card by the player's own non choice resources, then check borrowing to left and right
        // is legal and reduce the cost by the resources provided there too.
        let mut cost = card.cost().clone();
        self.reduce_by_own_resources(&mut cost);

        if !check(&borrowing.left, &visible_game.left_neighbour(), &mut cost) {
            return false;
        }
        if !(check(&borrowing.right, &visible_game.right_neighbour(), &mut cost)) {
            return false;
        }

        // We're left with our own choice cards. Ideally the borrowing definition would say exactly which own cards
        // we're using too, but it doesn't yet (and may never because it's expensive to add this information). So
        // iterate over all possible combinations to see if one works.
        let mut choices = vec![];
        add_choices(&self.built_structures, &cost, Source::Own, &mut choices);

        let mut combinations = 1;
        for choice in &choices {
            combinations *= choice.resources.len() as u32;
        }

        for combination in 0..combinations {
            let mut cost_copy = cost.clone();
            let mut c = combination;
            for choice in &choices {
                let len = choice.resources.len();
                let index = (c % len as u32) as usize;
                cost_copy -= &choice.resources[index];
                c /= len as u32;
            }
            if cost_copy.satisfied() {
                return true;
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
    /// If `single_option` is `true`, only a single option will be returned, even if multiple are possible. The option
    /// returned is selected at random from those available. This can be much more efficient if only a single option is
    /// required as we can stop as soon as we find a valid option.
    ///
    /// Note this function doesn't verify the cards the player has in their hand, meaning `card` can be a card the
    /// player doesn't have. As long as they can afford it, valid actions will be returned to achieve it.
    pub fn options_for_card(&self, card: &Card, visible_game: &VisibleGame, single_option: bool) -> ActionOptions {
        // Get the cost of the card, and subtract the Wonder starting resources and any non-choice resources owned by
        // the player.
        let mut cost = card.cost().clone();
        self.reduce_by_own_resources(&mut cost);
        if cost.satisfied() {
            // Can afford with own resources.
            return ActionOptions {
                actions: vec![Action::Build(*card, Borrowing::no_borrowing())],
            };
        }

        // We now add all choice cards owned by the player, and all borrowable resources owned by their neighbours, and
        // iterate over all possible combinations of those cards. We filter our entire cards that don't have the
        // resources we need, and filter choice cards to just the resources required.
        let mut choices = vec![];
        add_choices(&self.built_structures, &cost, Source::Own, &mut choices);
        let own_choices_count = choices.len();
        add_choices(
            &visible_game.left_neighbour().built_structures,
            &cost,
            Source::LeftNeighbour,
            &mut choices,
        );
        add_choices(
            &visible_game.right_neighbour().built_structures,
            &cost,
            Source::RightNeighbour,
            &mut choices,
        );

        // If returning a single option, shuffle the choices so we select the option returned at random. Own choices
        // must always come before neighbour choices, though, so we don't over-borrow.
        if single_option {
            choices[..own_choices_count].shuffle(&mut thread_rng());
            choices[own_choices_count..].shuffle(&mut thread_rng());
        }

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

            let mut left_borrowing = vec![];
            let mut right_borrowing = vec![];
            'outer: for combination in 0..combinations {
                let mut cost_copy = cost.clone();
                let mut c = combination;
                left_borrowing.clear();
                right_borrowing.clear();
                for choice in &choices {
                    // If own resource, always include it. If neighbours', also try without it.
                    let len = choice.resources.len() + if choice.source == Source::Own { 0 } else { 1 };
                    let index = (c % len as u32) as usize;
                    if choice.source == Source::Own {
                        cost_copy -= &choice.resources[index];
                    } else if index > 0 {
                        // TODO: cost of borrowing needs to vary depending on yellow cards.
                        if cost_copy.coins <= -2 {
                            if !cost_copy.has(&choice.resources[index - 1]) {
                                // We already have enough of whatever this option provides. Therefore, this particular
                                // combination is not valid. Skip to the next.
                                continue 'outer;
                            }
                            cost_copy -= &choice.resources[index - 1];
                            cost_copy.coins += 2;
                            if choice.source == Source::LeftNeighbour {
                                left_borrowing.push(Borrow::new(choice.card, choice.resources[index - 1]));
                            } else {
                                right_borrowing.push(Borrow::new(choice.card, choice.resources[index - 1]));
                            }
                        } else {
                            // Out of money for borrowing.
                            continue 'outer;
                        }
                    }
                    c /= len as u32;
                }
                if cost_copy.satisfied() {
                    actions.push(Action::Build(
                        *card,
                        Borrowing::new(left_borrowing.clone(), right_borrowing.clone()),
                    ));
                    if single_option {
                        break 'outer;
                    }
                }
            }
        }

        ActionOptions { actions }
    }

    /// Reduces `cost` by the resources provided by this player's built structures, their coins, and their wonder's
    /// starting resource. "Choice" resources are not used.
    fn reduce_by_own_resources(&self, cost: &mut Cost) {
        *cost -= &self.wonder.starting_resource();
        cost.coins -= self.coins;
        for card in &self.built_structures {
            if let Power::Producer(produced_resources) | Power::PurchasableProducer(produced_resources) = card.power() {
                match produced_resources {
                    ProducedResources::Single(resource) => *cost -= resource,
                    ProducedResources::Double(resource) => {
                        *cost -= resource;
                        *cost -= resource;
                    }
                    ProducedResources::Choice(_) => {}
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Source {
    Own,
    LeftNeighbour,
    RightNeighbour,
}

struct UsableResources {
    card: Card,
    resources: Vec<Resource>,
    source: Source,
}

/// Given some own cards or a neighbour's cards, adds to `choices` the things we need to consider in order to
/// find all possible ways of achieving the cost. Cards that provide only resources we don't need are removed
/// entirely. Cards that provide options of resources are reduced to only those resources we require.
fn add_choices(cards: &[Card], cost: &Cost, source: Source, choices: &mut Vec<UsableResources>) {
    for card in cards {
        match (card.power(), source) {
            // Make sure we only borrow brown and grey cards from neighbours (not yellow).
            (Power::Producer(produced_resources), Source::Own)
            | (Power::PurchasableProducer(produced_resources), _) => {
                match produced_resources {
                    ProducedResources::Single(resource) => {
                        // Filter out single choice own cards as we'll have already dealt with these. Only
                        // include the card if it has a resource we need.
                        if source != Source::Own && cost.has(resource) {
                            choices.push(UsableResources {
                                card: *card,
                                resources: vec![*resource],
                                source,
                            });
                        }
                    }
                    ProducedResources::Double(resource) => {
                        // Filter out single choice own cards as we'll have already dealt with these. Add two
                        // copies of the card so we can choose to use one resource or both.
                        if source != Source::Own && cost.has(resource) {
                            for _ in 0..2 {
                                choices.push(UsableResources {
                                    card: *card,
                                    resources: vec![*resource],
                                    source,
                                });
                            }
                        }
                    }
                    ProducedResources::Choice(resources) => {
                        // Filter the choices to only those we need.
                        let resources: Vec<Resource> = resources.iter().filter(|r| cost.has(r)).cloned().collect();
                        if !resources.is_empty() {
                            choices.push(UsableResources {
                                card: *card,
                                resources,
                                source,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
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
                .options_for_card(&Stockade, &visible_game(&players()), false)
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
                .options_for_card(&TreeFarm, &visible_game(&players()), false)
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
                .options_for_card(&Barracks, &visible_game(&players()), false)
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
                .options_for_card(&TreeFarm, &visible_game(&players()), false)
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
                .options_for_card(&LumberYard, &visible_game(&players()), false)
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
                .options_for_card(&Stockade, &visible_game(&players()), false)
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
                .options_for_card(&Temple, &visible_game(&players()), false)
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
                .options_for_card(&Stockade, &visible_game(&players()), false)
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
                .options_for_card(&Caravansery, &visible_game(&players()), false)
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
                .options_for_card(&Stockade, &visible_game(&public_players), false)
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
                .options_for_card(&Stockade, &visible_game(&public_players), false)
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
                .options_for_card(&Caravansery, &visible_game(&public_players), false)
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
                .options_for_card(&Caravansery, &visible_game(&public_players), false)
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
                .options_for_card(&Stockade, &visible_game(&public_players), false)
                .actions
                .len()
        );
    }

    #[test]
    fn options_for_card_returns_one_option_if_requested() {
        // Stockade requires 1 wood, we can borrow from either neighbour, resulting in two options, but we ask for just
        //one to be returned.
        let player = new_player(vec![]);
        let public_players = players_with_resources(vec![LumberYard], vec![TreeFarm]);
        assert_eq!(
            1,
            player
                .options_for_card(&Stockade, &visible_game(&public_players), true)
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
                .options_for_card(&Caravansery, &visible_game(&public_players), false)
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
                .options_for_card(&Stockade, &visible_game(&public_players), false)
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
                .options_for_card(&Stockade, &visible_game(&public_players), false)
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
                .options_for_card(&Laboratory, &visible_game(&public_players), false)
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
                .options_for_card(&Aqueduct, &visible_game(&public_players), false)
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
                .options_for_card(&Stockade, &visible_game(&public_players), false)
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
            player
                .options_for_card(&Baths, &visible_game(&players()), false)
                .actions
                .len()
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
    fn can_play_returns_false_if_borrowing_required_but_not_specified() {
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
    fn can_play_returns_false_if_borrowing_not_possible() {
        // Stockade requires 1 wood, we say we'll borrow from a neighbour, but the neighbour doesn't have the card.
        let player = new_player(vec![Stockade]);
        assert_eq!(
            false,
            player.can_play(
                &Action::Build(
                    Stockade,
                    Borrowing::new(vec![Borrow::new(LumberYard, Resource::Wood)], vec![])
                ),
                &visible_game(&players())
            )
        );
    }

    #[test]
    fn can_play_returns_false_if_borrowing_not_possible2() {
        // Archery range requires 2 wood (and 1 ore), we say we'll borrow the wood from a neighbour, but the neighbour
        // only has one wood.
        let mut player = new_player(vec![ArcheryRange, OreVein]);
        player.coins = 4;
        build(&mut player, OreVein);
        let public_players = players_with_resources(vec![LumberYard], vec![]);
        assert_eq!(
            false,
            player.can_play(
                &Action::Build(
                    ArcheryRange,
                    Borrowing::new(
                        vec![
                            Borrow::new(LumberYard, Resource::Wood),
                            Borrow::new(LumberYard, Resource::Wood)
                        ],
                        vec![]
                    )
                ),
                &visible_game(&public_players)
            )
        );
    }

    #[test]
    fn can_play_returns_false_if_borrowing_not_possible3() {
        // Stockade requires 1 wood, we say we'll borrow from a neighbour, but don't have enough coins.
        let mut player = new_player(vec![Stockade]);
        player.coins = 1;
        let public_players = players_with_resources(vec![LumberYard], vec![]);
        assert_eq!(
            false,
            player.can_play(
                &Action::Build(
                    Stockade,
                    Borrowing::new(vec![Borrow::new(LumberYard, Resource::Wood)], vec![])
                ),
                &visible_game(&public_players)
            )
        );
    }

    #[test]
    fn can_play_returns_false_if_borrowing_not_possible4() {
        // Stockade requires 1 wood, we say we'll borrow from a neighbour, but the card we try to borrow doesn't produce
        // wood.
        let mut player = new_player(vec![Stockade]);
        player.coins = 1;
        let public_players = players_with_resources(vec![StonePit], vec![]);
        assert_eq!(
            false,
            player.can_play(
                &Action::Build(
                    Stockade,
                    Borrowing::new(vec![Borrow::new(LumberYard, Resource::Wood)], vec![])
                ),
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
            turn: 0,
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
