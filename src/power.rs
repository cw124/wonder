//! Represents what a card or a wonder stage does for a player (for example, delivers victory points, or gives access to
//! a scientific structure).

use std::fmt;
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use strum_macros::EnumIter;

use crate::card::{Card, Colour};
use crate::resources::Resources;
use crate::utils::plural;

/// Represents what a card or a wonder stage does for a player (for example, delivers victory points, or gives access to
/// a scientific structure).
pub enum Power {
    /// Produces resources that are purchasable by a neighbour (ie. brown and grey cards). The vector contains a single
    /// [`Resources`] item for normal resources, or more than one to represent a choice of resources.
    PurchasableProducer(Vec<Resources>),
    /// Produces resources that are not purchasable by a neighbour. The vector contains a single [`Resources`] item for
    /// normal resources, or more than one to represent a choice of resources.
    Producer(Vec<Resources>),
    /// Provides victory points.
    VictoryPoints(u32),
    /// Provides coins.
    Coins(u32),
    /// Allows the player to buy brown card resources from their anti-clockwise neighbour for 1 coin rather than 2.
    BuyBrownAntiClockwise,
    /// Allows the player to buy brown card resources from their clockwise neighbour for 1 coin rather than 2.
    BuyBrownClockwise,
    /// Allows the player to buy grey card resources from either neighbour for 1 coin rather than 2.
    BuyGrey,
    /// A choice of _one_ of the given [`ScienceItem`]s
    Science(Vec<ScienceItem>),
    /// Provides shields.
    Shields(u32),
    /// Provides coins and/or victory points based on the number of game items a player or his neighbours have. For
    /// example, provides victory points based on the number of brown cards the player's neighbours have.
    PerGameItemRewards(Vec<PerGameItemReward>),
}

impl Power {
    /// Convenience method that returns a [`Power::PerGameItemRewards`] matching on the given card colour
    ///
    /// # Examples
    ///
    /// ```
    /// Power::per_card_reward(Colour::Yellow, true, false, 1, 1)
    /// ```
    ///
    /// Returns a [`Power`] that awards one coin and one victory point per yellow card built by the player only (not his
    /// neighbours) -- in other words, the power of the [`Card::Lighthouse`].
    pub fn per_card_reward(
                colour: Colour,
                me: bool,
                neighbours: bool,
                coins_per_thing: u32,
                points_per_thing: u32) -> Power {

        Power::PerGameItemRewards(vec![PerGameItemReward {
            game_item: Box::new(move |game_item| matches!(game_item,
                    CountableGameItem::CountableCard(card) if card.colour() == colour)),
            me,
            neighbours,
            coins_per_thing,
            points_per_thing
        }])
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Power::PurchasableProducer(resources) => format!("{}", resources.iter().format(" or ")),
            Power::Producer(resources) => format!("{}", resources.iter().format(" or ")),
            Power::VictoryPoints(points) => plural(*points as i32, "VP"),
            Power::Coins(coins) => plural(*coins as i32, "coin"),
            Power::BuyBrownAntiClockwise => "Buy brown cards for 1 coin anti-clockwise".to_string(),
            Power::BuyBrownClockwise => "Buy brown cards for 1 coin clockwise".to_string(),
            Power::BuyGrey => "Buy grey cards for 1 coin".to_string(),
            Power::Science(symbol) => format!("{}", symbol.iter().map(|symbol| format!("{} symbol", symbol)).format(" or ")),
            Power::Shields(shields) => plural(*shields as i32, "shield"),
            Power::PerGameItemRewards(_) => "Per game item thing (TODO)".to_string()  // TODO
        })
    }
}

/// Represents the three different symbols found on Science (ie. green) cards.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
pub enum ScienceItem {
    Compass,
    Cog,
    Tablet
}

impl Display for ScienceItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ScienceItem::Compass => "Compass",
            ScienceItem::Cog => "Cog",
            ScienceItem::Tablet => "Tablet"
        })
    }
}

/// Provides coins and/or victory points based on the number of game items a player or his neighbours have. For example,
/// provides victory points based on the number of brown cards the player's neighbours have.
pub struct PerGameItemReward {
    /// A function or closure that returns true if the given [`CountableGameItem`] is one of the things counted by this
    /// reward. For example, it might return true if the `CountableGameItem` was a brown card.
    pub game_item: Box<dyn Fn(CountableGameItem) -> bool>,
    /// True if the player's items should be counted.
    pub me: bool,
    /// True if the player's neighbours' items should be counted.
    pub neighbours: bool,
    pub coins_per_thing: u32,
    pub points_per_thing: u32
}

/// Something in the game that is "countable", such as the number of cards a player has built, or the number of Defeat
/// Tokens they have.
#[allow(dead_code)]
pub enum CountableGameItem {
    CountableCard(Card),
    DefeatToken,
    CompletedWonderStage,
}