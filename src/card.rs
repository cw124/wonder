use std::fmt::{Display, Formatter};
use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::power::{CountableGameItem, ScienceItem};
use crate::power::PerGameItemReward;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[allow(dead_code)]
pub enum Card {

    // Age 1
    // =====

    // Brown
    LumberYard,
    StonePit,
    ClayPool,
    OreVein,
    TreeFarm,
    Excavation,
    ClayPit,
    TimberYard,
    ForestCave,
    Mine,

    // Grey
    Loom1,
    Glassworks1,
    Press1,

    // Blue
    Pawnshop,
    Baths,
    Altar,
    Theater,

    // Yellow
    Tavern,
    EastTradingPost,
    WestTradingPost,
    Marketplace,

    // Green
    Apothecary,
    Workshop,
    Scriptorium,

    // Red
    Stockade,
    Barracks,
    GuardTower,


    // Age 2
    // =====

    // Brown
    Sawmill,
    Quarry,
    Brickyard,
    Foundry,

    // Grey
    Loom2,
    Glassworks2,
    Press2,

    // Blue
    Aqueduct,
    Temple,
    Statue,
    Courthouse,

    // Yellow
    Forum,
    Caravansery,
    Vineyard,
    Bazar,

    // Green
    Dispensary,
    Laboratory,
    Library,
    School,

    // Red
    Walls,
    TrainingGround,
    Stables,
    ArcheryRange,


    // Age 3
    // =====

    // Blue
    Pantheon,
    Gardens,
    TownHall,
    Palace,
    Senate,

    // Yellow
    Haven,
    Lighthouse,
    ChamberOfCommerce,
    Arena,

    // Green
    Lodge,
    Observatory,
    University,
    Academy,
    Study,

    // Red
    Fortifications,
    Circus,
    Arsenal,
    SiegeWorkshop,

    // Purple
    WorkersGuild,
    CraftsmensGuild,
    TradersGuild,
    PhilosophersGuild,
    SpiesGuild,
    StrategistsGuild,
    ShipownersGuild,
    ScientistsGuild,
    MagistratesGuild,
    BuildersGuild,
}

#[derive(Debug, PartialEq)]
pub enum Age {
    First,
    Second,
    Third
}

#[derive(PartialEq)]
pub enum Colour {
    Brown,
    Grey,
    Blue,
    Yellow,
    Red,
    Green,
    Purple,
}

#[allow(dead_code)]
struct CardInfo<'a> {
    name: &'a str,
    age: Age,
    players_needed: Vec<u32>,
    cost: Resources,
    chains_to: Vec<Card>,
    colour: Colour,
    power: Power,
}

#[allow(dead_code)]
impl Card {
    fn info(&self) -> CardInfo {
        match self {
            Card::LumberYard => CardInfo {
                name: "Lumber Yard",
                age: Age::First,
                players_needed: vec![3, 4],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::wood(1))),
            },

            Card::StonePit => CardInfo {
                name: "Stone Pit",
                age: Age::First,
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::stone(1))),
            },

            Card::ClayPool => CardInfo {
                name: "Clay Pool",
                age: Age::First,
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::clay(1))),
            },

            Card::OreVein => CardInfo {
                name: "Ore Vein",
                age: Age::First,
                players_needed: vec![3, 4],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::ore(1))),
            },

            Card::TreeFarm => CardInfo {
                name: "Tree Farm",
                age: Age::First,
                players_needed: vec![6],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::wood(1),
                    Resources::clay(1)])),
            },

            Card::Excavation => CardInfo {
                name: "Excavation",
                age: Age::First,
                players_needed: vec![4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::stone(1),
                    Resources::clay(1)])),
            },

            Card::ClayPit => CardInfo {
                name: "Clay Pit",
                age: Age::First,
                players_needed: vec![3],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::clay(1),
                    Resources::ore(1)])),
            },

            Card::TimberYard => CardInfo {
                name: "Timber Yard",
                age: Age::First,
                players_needed: vec![3],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::stone(1),
                    Resources::wood(1)])),
            },

            Card::ForestCave => CardInfo {
                name: "Forest Cave",
                age: Age::First,
                players_needed: vec![5],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::wood(1),
                    Resources::ore(1)])),
            },

            Card::Mine => CardInfo {
                name: "Mine",
                age: Age::First,
                players_needed: vec![6],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::ore(1),
                    Resources::stone(1)])),
            },

            Card::Loom1 => CardInfo {
                name: "Loom",
                age: Age::First,
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::loom(1))),
            },

            Card::Glassworks1 => CardInfo {
                name: "Glassworks",
                age: Age::First,
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::glass(1))),
            },

            Card::Press1 => CardInfo {
                name: "Press",
                age: Age::First,
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::papyrus(1))),
            },

            Card::Pawnshop => CardInfo {
                name: "Pawnshop",
                age: Age::First,
                players_needed: vec![4, 7],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Baths => CardInfo {
                name: "Baths",
                age: Age::First,
                players_needed: vec![3, 7],
                cost: Resources::stone(1),
                chains_to: vec![Card::Aqueduct],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Altar => CardInfo {
                name: "Altar",
                age: Age::First,
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![Card::Temple],
                colour: Colour::Blue,
                power: Power::VictoryPoints(2),
            },

            Card::Theater => CardInfo {
                name: "Theater",
                age: Age::First,
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![Card::Statue],
                colour: Colour::Blue,
                power: Power::VictoryPoints(2),
            },

            Card::Tavern => CardInfo {
                name: "Tavern",
                age: Age::First,
                players_needed: vec![4, 5, 7],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::Coins(5),
            },

            Card::EastTradingPost => CardInfo {
                name: "East Trading Post",
                age: Age::First,
                players_needed: vec![3, 7],
                cost: Resources::free(),
                chains_to: vec![Card::Forum],
                colour: Colour::Yellow,
                power: Power::BuyBrownAntiClockwise,
            },

            Card::WestTradingPost => CardInfo {
                name: "West Trading Post",
                age: Age::First,
                players_needed: vec![3, 7],
                cost: Resources::free(),
                chains_to: vec![Card::Forum],
                colour: Colour::Yellow,
                power: Power::BuyBrownClockwise,
            },

            Card::Marketplace => CardInfo {
                name: "Marketplace",
                age: Age::First,
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![Card::Caravansery],
                colour: Colour::Yellow,
                power: Power::BuyGrey,
            },

            Card::Apothecary => CardInfo {
                name: "Apothecary",
                age: Age::First,
                players_needed: vec![3, 5],
                cost: Resources::loom(1),
                chains_to: vec![Card::Stables, Card::Dispensary],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Workshop => CardInfo {
                name: "Workshop",
                age: Age::First,
                players_needed: vec![3, 7],
                cost: Resources::glass(1),
                chains_to: vec![Card::ArcheryRange, Card::Laboratory],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::Scriptorium => CardInfo {
                name: "Scriptorium",
                age: Age::First,
                players_needed: vec![3, 4],
                cost: Resources::papyrus(1),
                chains_to: vec![Card::Courthouse, Card::Library],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::Stockade => CardInfo {
                name: "Stockade",
                age: Age::First,
                players_needed: vec![3, 7],
                cost: Resources::wood(1),
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(1),
            },

            Card::Barracks => CardInfo {
                name: "Barracks",
                age: Age::First,
                players_needed: vec![3, 5],
                cost: Resources::ore(1),
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(1),
            },

            Card::GuardTower => CardInfo {
                name: "Guard Tower",
                age: Age::First,
                players_needed: vec![3, 4],
                cost: Resources::clay(1),
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(1),
            },

            Card::Sawmill => CardInfo {
                name: "Sawmill",
                age: Age::Second,
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::wood(2))),
            },

            Card::Quarry => CardInfo {
                name: "Quarry",
                age: Age::Second,
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::stone(2))),
            },

            Card::Brickyard => CardInfo {
                name: "Brickyard",
                age: Age::Second,
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::clay(2))),
            },

            Card::Foundry => CardInfo {
                name: "Foundry",
                age: Age::Second,
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::ore(2))),
            },

            Card::Loom2 => CardInfo {
                name: "Loom",
                age: Age::Second,
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::loom(1))),
            },

            Card::Glassworks2 => CardInfo {
                name: "Glassworks",
                age: Age::Second,
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::glass(1))),
            },

            Card::Press2 => CardInfo {
                name: "Press",
                age: Age::Second,
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::papyrus(1))),
            },

            Card::Aqueduct => CardInfo {
                name: "Aqueduct",
                age: Age::Second,
                players_needed: vec![3, 7],
                cost: Resources::stone(3),
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(5),
            },

            Card::Temple => CardInfo {
                name: "Temple",
                age: Age::Second,
                players_needed: vec![3, 6],
                cost: Resources { wood: 1, clay: 1, glass: 1, ..Default::default() },
                chains_to: vec![Card::Pantheon],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Statue => CardInfo {
                name: "Statue",
                age: Age::Second,
                players_needed: vec![3, 7],
                cost: Resources { wood: 1, ore: 2, ..Default::default() },
                chains_to: vec![Card::Gardens],
                colour: Colour::Blue,
                power: Power::VictoryPoints(4),
            },

            Card::Courthouse => CardInfo {
                name: "Courthouse",
                age: Age::Second,
                players_needed: vec![3, 5],
                cost: Resources { clay: 2, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(4),
            },

            Card::Forum => CardInfo {
                name: "Forum",
                age: Age::Second,
                players_needed: vec![3, 6, 7],
                cost: Resources::clay(2),
                chains_to: vec![Card::Haven],
                colour: Colour::Yellow,
                power: Power::Producer(ProducedResources::Choice(vec![
                    Resources::loom(1),
                    Resources::glass(1),
                    Resources::papyrus(1)])),
            },

            Card::Caravansery => CardInfo {
                name: "Caravansery",
                age: Age::Second,
                players_needed: vec![3, 5, 6],
                cost: Resources::wood(2),
                chains_to: vec![Card::Lighthouse],
                colour: Colour::Yellow,
                power: Power::Producer(ProducedResources::Choice(vec![
                    Resources::wood(1),
                    Resources::stone(1),
                    Resources::ore(1),
                    Resources::clay(1)])),
            },

            Card::Vineyard => CardInfo {
                name: "Vineyard",
                age: Age::Second,
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Brown, true, true, 1, 0),
            },

            Card::Bazar => CardInfo {
                name: "Bazar",
                age: Age::Second,
                players_needed: vec![4, 7],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Grey, true, true, 2, 0),
            },

            Card::Dispensary => CardInfo {
                name: "Dispensary",
                age: Age::Second,
                players_needed: vec![3, 4],
                cost: Resources { ore: 2, glass: 1, ..Default::default() },
                chains_to: vec![Card::Arena, Card::Lodge],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Laboratory => CardInfo {
                name: "Laboratory",
                age: Age::Second,
                players_needed: vec![3, 5],
                cost: Resources { clay: 2, papyrus: 1, ..Default::default() },
                chains_to: vec![Card::SiegeWorkshop, Card::Observatory],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::Library => CardInfo {
                name: "Library",
                age: Age::Second,
                players_needed: vec![3, 6],
                cost: Resources { stone: 2, loom: 1, ..Default::default() },
                chains_to: vec![Card::Senate, Card::University],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::School => CardInfo {
                name: "School",
                age: Age::Second,
                players_needed: vec![3, 7],
                cost: Resources { wood: 1, papyrus: 1, ..Default::default() },
                chains_to: vec![Card::Academy, Card::Study],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::Walls => CardInfo {
                name: "Walls",
                age: Age::Second,
                players_needed: vec![3, 7],
                cost: Resources::stone(3),
                chains_to: vec![Card::Fortifications],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::TrainingGround => CardInfo {
                name: "Training Ground",
                age: Age::Second,
                players_needed: vec![4, 6, 7],
                cost: Resources { wood: 1, ore: 2, ..Default::default() },
                chains_to: vec![Card::Circus],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::Stables => CardInfo {
                name: "Stables",
                age: Age::Second,
                players_needed: vec![3, 5],
                cost: Resources { ore: 1, clay: 1, wood: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::ArcheryRange => CardInfo {
                name: "Archery Range",
                age: Age::Second,
                players_needed: vec![3, 6],
                cost: Resources { wood: 2, ore: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::Pantheon => CardInfo {
                name: "Pantheon",
                age: Age::Third,
                players_needed: vec![3, 6],
                cost: Resources { clay: 2, ore: 1, papyrus: 1, loom: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(7),
            },

            Card::Gardens => CardInfo {
                name: "Gardens",
                age: Age::Third,
                players_needed: vec![3, 4],
                cost: Resources { wood: 2, clay: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(5),
            },

            Card::TownHall => CardInfo {
                name: "Town Hall",
                age: Age::Third,
                players_needed: vec![3, 5, 6],
                cost: Resources { glass: 1, ore: 1, stone: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(6),
            },

            Card::Palace => CardInfo {
                name: "Palace",
                age: Age::Third,
                players_needed: vec![3, 7],
                cost: Resources { glass: 1, papyrus: 1, loom: 1, clay: 1, wood: 1, ore: 1, stone: 1, coins: 0 },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(8),
            },

            Card::Senate => CardInfo {
                name: "Senate",
                age: Age::Third,
                players_needed: vec![3, 5],
                cost: Resources { ore: 1, stone: 1, wood: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(6),
            },

            Card::Haven => CardInfo {
                name: "Haven",
                age: Age::Third,
                players_needed: vec![3, 4],
                cost: Resources { loom: 1, ore: 1, wood: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Brown, true, false, 1, 1),
            },

            Card::Lighthouse => CardInfo {
                name: "Lighthouse",
                age: Age::Third,
                players_needed: vec![3, 6],
                cost: Resources { glass: 1, stone: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Yellow, true, false, 1, 1),
            },

            Card::ChamberOfCommerce => CardInfo {
                name: "Chamber Of Commerce",
                age: Age::Third,
                players_needed: vec![4, 6],
                cost: Resources { clay: 2, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Grey, true, false, 2, 2),
            },

            Card::Arena => CardInfo {
                name: "Arena",
                age: Age::Third,
                players_needed: vec![3, 5, 7],
                cost: Resources { ore: 1, stone: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| matches!(game_item, CountableGameItem::CompletedWonderStage)),
                    me: true,
                    neighbours: false,
                    coins_per_thing: 3,
                    points_per_thing: 1,
                }]),
            },

            Card::Lodge => CardInfo {
                name: "Lodge",
                age: Age::Third,
                players_needed: vec![3, 6],
                cost: Resources { clay: 2, loom: 1, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Observatory => CardInfo {
                name: "Observatory",
                age: Age::Third,
                players_needed: vec![3, 7],
                cost: Resources { ore: 2, glass: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::University => CardInfo {
                name: "University",
                age: Age::Third,
                players_needed: vec![3, 4],
                cost: Resources { wood: 2, papyrus: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::Academy => CardInfo {
                name: "Academy",
                age: Age::Third,
                players_needed: vec![3, 7],
                cost: Resources { stone: 3, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Study => CardInfo {
                name: "Study",
                age: Age::Third,
                players_needed: vec![3, 5],
                cost: Resources { wood: 1, papyrus: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::Fortifications => CardInfo {
                name: "Fortifications",
                age: Age::Third,
                players_needed: vec![3, 7],
                cost: Resources { stone: 1, ore: 3, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::Circus => CardInfo {
                name: "Circus",
                age: Age::Third,
                players_needed: vec![4, 5, 6],
                cost: Resources { stone: 3, ore: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::Arsenal => CardInfo {
                name: "Arsenal",
                age: Age::Third,
                players_needed: vec![3, 4, 7],
                cost: Resources { ore: 1, wood: 2, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::SiegeWorkshop => CardInfo {
                name: "Siege Workshop",
                age: Age::Third,
                players_needed: vec![3, 5],
                cost: Resources { wood: 1, clay: 3, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::WorkersGuild => CardInfo {
                name: "Workers Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { ore: 2, clay: 1, stone: 1, wood: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Brown, false, true, 0, 1),
            },

            Card::CraftsmensGuild => CardInfo {
                name: "Craftsmens Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { ore: 2, stone: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Grey, false, true, 0, 2),
            },

            Card::TradersGuild => CardInfo {
                name: "Traders Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { loom: 1, papyrus: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Yellow, false, true, 0, 1),
            },

            Card::PhilosophersGuild => CardInfo {
                name: "Philosophers Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { clay: 3, loom: 1, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Green, false, true, 0, 1),
            },

            Card::SpiesGuild => CardInfo {
                name: "Spies Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { clay: 3, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Red, false, true, 0, 2),
            },

            Card::StrategistsGuild => CardInfo {
                name: "Strategists Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { ore: 2, stone: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| matches!(game_item, CountableGameItem::DefeatToken)),
                    me: false,
                    neighbours: true,
                    coins_per_thing: 0,
                    points_per_thing: 1,
                }]),
            },

            Card::ShipownersGuild => CardInfo {
                name: "Shipowners Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { wood: 3, papyrus: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| matches!(game_item,
                        CountableGameItem::CountableCard(card) if
                            card.info().colour == Colour::Brown ||
                            card.info().colour == Colour::Grey ||
                            card.info().colour == Colour::Purple)),
                    me: true,
                    neighbours: false,
                    coins_per_thing: 0,
                    points_per_thing: 1,
                }]),
            },

            Card::ScientistsGuild => CardInfo {
                name: "Scientists Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { wood: 2, ore: 2, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::Science(vec![ScienceItem::Compass, ScienceItem::Cog, ScienceItem::Tablet]),
            },

            Card::MagistratesGuild => CardInfo {
                name: "Migistrates Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { wood: 3, stone: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Blue, false, true, 0, 1),
            },

            Card::BuildersGuild => CardInfo {
                name: "Builders Guild",
                age: Age::Third,
                players_needed: vec![3],
                cost: Resources { stone: 2, clay: 2, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| matches!(game_item, CountableGameItem::CompletedWonderStage)),
                    me: true,
                    neighbours: true,
                    coins_per_thing: 0,
                    points_per_thing: 1,
                }]),
            },
        }
    }

    pub fn age(&self) -> Age {
        self.info().age
    }

    pub fn players_needed(&self) -> Vec<u32> {
        self.info().players_needed
    }

    pub fn cost(&self) -> Resources {
        self.info().cost
    }

    pub fn chains_to(&self) -> Vec<Card> {
        self.info().chains_to
    }

    pub fn colour(&self) -> Colour {
        self.info().colour
    }

    pub fn power(&self) -> Power {
        self.info().power
    }

    pub fn strength(&self) -> f32 {
        match self.power() {
            // TODO: can we write these four options more succinctly?
            Power::VictoryPoints(points) => points as f32,
            Power::Science(_) => 1.0,
            _ => 0.0
        }
    }

    fn coins_to_victory_points(coins: f32) -> f32 {
        coins / 3.0
    }
}


impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info().name)
    }
}


/// Creates a new, shuffled deck for the given age and number of players.
pub fn new_deck(age: Age, player_count: u32) -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    let mut guilds: Vec<Card> = vec![];

    // Add all cards with the correct age and number of players needed, added guilds to a separate vector for the time
    // being.
    for card in Card::iter() {
        if card.age() == age {
            for players_needed in card.players_needed() {
                if player_count >= players_needed {
                    if card.colour() == Colour::Purple {
                        guilds.push(card);
                    } else {
                        deck.push(card);
                    }
                }
            }
        }
    }

    // Shuffle the guilds separately and add player_count + 2 random ones to the deck.
    if age == Age::Third {
        guilds.shuffle(&mut thread_rng());
        for _i in 0..(player_count + 2) as usize {
            deck.push(guilds.pop().unwrap());
        }
    }

    // Shuffle the complete deck and return it.
    deck.shuffle(&mut thread_rng());
    deck
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_has_right_number_of_cards() {
        assert_eq!(21, new_deck(Age::First, 3).len());
        assert_eq!(28, new_deck(Age::First, 4).len());
        assert_eq!(35, new_deck(Age::First, 5).len());
        assert_eq!(42, new_deck(Age::First, 6).len());
        assert_eq!(49, new_deck(Age::First, 7).len());

        assert_eq!(21, new_deck(Age::Second, 3).len());
        assert_eq!(28, new_deck(Age::Second, 4).len());
        assert_eq!(35, new_deck(Age::Second, 5).len());
        assert_eq!(42, new_deck(Age::Second, 6).len());
        assert_eq!(49, new_deck(Age::Second, 7).len());

        assert_eq!(21, new_deck(Age::Third, 3).len());
        assert_eq!(28, new_deck(Age::Third, 4).len());
        assert_eq!(35, new_deck(Age::Third, 5).len());
        assert_eq!(42, new_deck(Age::Third, 6).len());
        assert_eq!(49, new_deck(Age::Third, 7).len());
    }

    #[test]
    fn no_second_or_third_age_cards_in_first_age_deck() {
        assert!(!new_deck(Age::First, 3).contains(&Card::Sawmill));
        assert!(!new_deck(Age::First, 3).contains(&Card::Pantheon));
    }
}
