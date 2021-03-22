use std::fmt;
use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::power::Power;
use crate::power::{CountableGameItem, ScienceItem};
use crate::power::{PerGameItemReward, ProducedResources};
use crate::resources::Cost;
use crate::resources::Resource;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
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
    Third,
}

#[derive(PartialEq, Eq, Hash)]
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
    cost: Cost,
    chains_to: Vec<Card>,
    colour: Colour,
    power: Power,
}

lazy_static! {
    static ref LUMBER_YARD: CardInfo<'static> = CardInfo {
        name: "Lumber Yard",
        age: Age::First,
        players_needed: vec![3, 4],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Wood)),
    };
    static ref STONE_PIT: CardInfo<'static> = CardInfo {
        name: "Stone Pit",
        age: Age::First,
        players_needed: vec![3, 5],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Stone)),
    };
    static ref CLAY_POOL: CardInfo<'static> = CardInfo {
        name: "Clay Pool",
        age: Age::First,
        players_needed: vec![3, 5],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Clay)),
    };
    static ref ORE_VEIN: CardInfo<'static> = CardInfo {
        name: "Ore Vein",
        age: Age::First,
        players_needed: vec![3, 4],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Ore)),
    };
    static ref TREE_FARM: CardInfo<'static> = CardInfo {
        name: "Tree Farm",
        age: Age::First,
        players_needed: vec![6],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Wood, Resource::Clay])),
    };
    static ref EXCAVATION: CardInfo<'static> = CardInfo {
        name: "Excavation",
        age: Age::First,
        players_needed: vec![4],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Stone, Resource::Clay])),
    };
    static ref CLAY_PIT: CardInfo<'static> = CardInfo {
        name: "Clay Pit",
        age: Age::First,
        players_needed: vec![3],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Clay, Resource::Ore])),
    };
    static ref TIMBER_YARD: CardInfo<'static> = CardInfo {
        name: "Timber Yard",
        age: Age::First,
        players_needed: vec![3],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Stone, Resource::Wood])),
    };
    static ref FOREST_CAVE: CardInfo<'static> = CardInfo {
        name: "Forest Cave",
        age: Age::First,
        players_needed: vec![5],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Wood, Resource::Ore])),
    };
    static ref MINE: CardInfo<'static> = CardInfo {
        name: "Mine",
        age: Age::First,
        players_needed: vec![6],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Ore, Resource::Stone])),
    };
    static ref LOOM1: CardInfo<'static> = CardInfo {
        name: "Loom",
        age: Age::First,
        players_needed: vec![3, 6],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Grey,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Loom)),
    };
    static ref GLASSWORKS1: CardInfo<'static> = CardInfo {
        name: "Glassworks",
        age: Age::First,
        players_needed: vec![3, 6],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Grey,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Glass)),
    };
    static ref PRESS1: CardInfo<'static> = CardInfo {
        name: "Press",
        age: Age::First,
        players_needed: vec![3, 6],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Grey,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Papyrus)),
    };
    static ref PAWNSHOP: CardInfo<'static> = CardInfo {
        name: "Pawnshop",
        age: Age::First,
        players_needed: vec![4, 7],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(3),
    };
    static ref BATHS: CardInfo<'static> = CardInfo {
        name: "Baths",
        age: Age::First,
        players_needed: vec![3, 7],
        cost: Cost::stone(1),
        chains_to: vec![Card::Aqueduct],
        colour: Colour::Blue,
        power: Power::VictoryPoints(3),
    };
    static ref ALTAR: CardInfo<'static> = CardInfo {
        name: "Altar",
        age: Age::First,
        players_needed: vec![3, 5],
        cost: Cost::free(),
        chains_to: vec![Card::Temple],
        colour: Colour::Blue,
        power: Power::VictoryPoints(2),
    };
    static ref THEATER: CardInfo<'static> = CardInfo {
        name: "Theater",
        age: Age::First,
        players_needed: vec![3, 6],
        cost: Cost::free(),
        chains_to: vec![Card::Statue],
        colour: Colour::Blue,
        power: Power::VictoryPoints(2),
    };
    static ref TAVERN: CardInfo<'static> = CardInfo {
        name: "Tavern",
        age: Age::First,
        players_needed: vec![4, 5, 7],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::Coins(5),
    };
    static ref EAST_TRADING_POST: CardInfo<'static> = CardInfo {
        name: "East Trading Post",
        age: Age::First,
        players_needed: vec![3, 7],
        cost: Cost::free(),
        chains_to: vec![Card::Forum],
        colour: Colour::Yellow,
        power: Power::BuyBrownAntiClockwise,
    };
    static ref WEST_TRADING_POST: CardInfo<'static> = CardInfo {
        name: "West Trading Post",
        age: Age::First,
        players_needed: vec![3, 7],
        cost: Cost::free(),
        chains_to: vec![Card::Forum],
        colour: Colour::Yellow,
        power: Power::BuyBrownClockwise,
    };
    static ref MARKETPLACE: CardInfo<'static> = CardInfo {
        name: "Marketplace",
        age: Age::First,
        players_needed: vec![3, 6],
        cost: Cost::free(),
        chains_to: vec![Card::Caravansery],
        colour: Colour::Yellow,
        power: Power::BuyGrey,
    };
    static ref APOTHECARY: CardInfo<'static> = CardInfo {
        name: "Apothecary",
        age: Age::First,
        players_needed: vec![3, 5],
        cost: Cost::loom(1),
        chains_to: vec![Card::Stables, Card::Dispensary],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Compass]),
    };
    static ref WORKSHOP: CardInfo<'static> = CardInfo {
        name: "Workshop",
        age: Age::First,
        players_needed: vec![3, 7],
        cost: Cost::glass(1),
        chains_to: vec![Card::ArcheryRange, Card::Laboratory],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Cog]),
    };
    static ref SCRIPTORIUM: CardInfo<'static> = CardInfo {
        name: "Scriptorium",
        age: Age::First,
        players_needed: vec![3, 4],
        cost: Cost::papyrus(1),
        chains_to: vec![Card::Courthouse, Card::Library],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Tablet]),
    };
    static ref STOCKADE: CardInfo<'static> = CardInfo {
        name: "Stockade",
        age: Age::First,
        players_needed: vec![3, 7],
        cost: Cost::wood(1),
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(1),
    };
    static ref BARRACKS: CardInfo<'static> = CardInfo {
        name: "Barracks",
        age: Age::First,
        players_needed: vec![3, 5],
        cost: Cost::ore(1),
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(1),
    };
    static ref GUARD_TOWER: CardInfo<'static> = CardInfo {
        name: "Guard Tower",
        age: Age::First,
        players_needed: vec![3, 4],
        cost: Cost::clay(1),
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(1),
    };
}

lazy_static! {
    static ref SAWMILL: CardInfo<'static> = CardInfo {
        name: "Sawmill",
        age: Age::Second,
        players_needed: vec![3, 4],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Double(Resource::Wood)),
    };
    static ref QUARRY: CardInfo<'static> = CardInfo {
        name: "Quarry",
        age: Age::Second,
        players_needed: vec![3, 4],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Double(Resource::Stone)),
    };
    static ref BRICKYARD: CardInfo<'static> = CardInfo {
        name: "Brickyard",
        age: Age::Second,
        players_needed: vec![3, 4],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Double(Resource::Clay)),
    };
    static ref FOUNDRY: CardInfo<'static> = CardInfo {
        name: "Foundry",
        age: Age::Second,
        players_needed: vec![3, 4],
        cost: Cost::coins(1),
        chains_to: vec![],
        colour: Colour::Brown,
        power: Power::PurchasableProducer(ProducedResources::Double(Resource::Ore)),
    };
    static ref LOOM2: CardInfo<'static> = CardInfo {
        name: "Loom",
        age: Age::Second,
        players_needed: vec![3, 5],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Grey,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Loom)),
    };
    static ref GLASSWORKS2: CardInfo<'static> = CardInfo {
        name: "Glassworks",
        age: Age::Second,
        players_needed: vec![3, 5],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Grey,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Glass)),
    };
    static ref PRESS2: CardInfo<'static> = CardInfo {
        name: "Press",
        age: Age::Second,
        players_needed: vec![3, 5],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Grey,
        power: Power::PurchasableProducer(ProducedResources::Single(Resource::Papyrus)),
    };
    static ref AQUEDUCT: CardInfo<'static> = CardInfo {
        name: "Aqueduct",
        age: Age::Second,
        players_needed: vec![3, 7],
        cost: Cost::stone(3),
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(5),
    };
    static ref TEMPLE: CardInfo<'static> = CardInfo {
        name: "Temple",
        age: Age::Second,
        players_needed: vec![3, 6],
        cost: Cost {
            wood: 1,
            clay: 1,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![Card::Pantheon],
        colour: Colour::Blue,
        power: Power::VictoryPoints(3),
    };
    static ref STATUE: CardInfo<'static> = CardInfo {
        name: "Statue",
        age: Age::Second,
        players_needed: vec![3, 7],
        cost: Cost {
            wood: 1,
            ore: 2,
            ..Default::default()
        },
        chains_to: vec![Card::Gardens],
        colour: Colour::Blue,
        power: Power::VictoryPoints(4),
    };
    static ref COURTHOUSE: CardInfo<'static> = CardInfo {
        name: "Courthouse",
        age: Age::Second,
        players_needed: vec![3, 5],
        cost: Cost {
            clay: 2,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(4),
    };
    static ref FORUM: CardInfo<'static> = CardInfo {
        name: "Forum",
        age: Age::Second,
        players_needed: vec![3, 6, 7],
        cost: Cost::clay(2),
        chains_to: vec![Card::Haven],
        colour: Colour::Yellow,
        power: Power::Producer(ProducedResources::Choice(vec![
            Resource::Loom,
            Resource::Glass,
            Resource::Papyrus
        ])),
    };
    static ref CARAVANSERY: CardInfo<'static> = CardInfo {
        name: "Caravansery",
        age: Age::Second,
        players_needed: vec![3, 5, 6],
        cost: Cost::wood(2),
        chains_to: vec![Card::Lighthouse],
        colour: Colour::Yellow,
        power: Power::Producer(ProducedResources::Choice(vec![
            Resource::Wood,
            Resource::Stone,
            Resource::Ore,
            Resource::Clay,
        ])),
    };
    static ref VINEYARD: CardInfo<'static> = CardInfo {
        name: "Vineyard",
        age: Age::Second,
        players_needed: vec![3, 6],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::per_card_reward(Colour::Brown, true, true, 1, 0),
    };
    static ref BAZAR: CardInfo<'static> = CardInfo {
        name: "Bazar",
        age: Age::Second,
        players_needed: vec![4, 7],
        cost: Cost::free(),
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::per_card_reward(Colour::Grey, true, true, 2, 0),
    };
    static ref DISPENSARY: CardInfo<'static> = CardInfo {
        name: "Dispensary",
        age: Age::Second,
        players_needed: vec![3, 4],
        cost: Cost {
            ore: 2,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![Card::Arena, Card::Lodge],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Compass]),
    };
    static ref LABORATORY: CardInfo<'static> = CardInfo {
        name: "Laboratory",
        age: Age::Second,
        players_needed: vec![3, 5],
        cost: Cost {
            clay: 2,
            papyrus: 1,
            ..Default::default()
        },
        chains_to: vec![Card::SiegeWorkshop, Card::Observatory],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Cog]),
    };
    static ref LIBRARY: CardInfo<'static> = CardInfo {
        name: "Library",
        age: Age::Second,
        players_needed: vec![3, 6],
        cost: Cost {
            stone: 2,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![Card::Senate, Card::University],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Tablet]),
    };
    static ref SCHOOL: CardInfo<'static> = CardInfo {
        name: "School",
        age: Age::Second,
        players_needed: vec![3, 7],
        cost: Cost {
            wood: 1,
            papyrus: 1,
            ..Default::default()
        },
        chains_to: vec![Card::Academy, Card::Study],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Tablet]),
    };
    static ref WALLS: CardInfo<'static> = CardInfo {
        name: "Walls",
        age: Age::Second,
        players_needed: vec![3, 7],
        cost: Cost::stone(3),
        chains_to: vec![Card::Fortifications],
        colour: Colour::Red,
        power: Power::Shields(2),
    };
    static ref TRAINING_GROUND: CardInfo<'static> = CardInfo {
        name: "Training Ground",
        age: Age::Second,
        players_needed: vec![4, 6, 7],
        cost: Cost {
            wood: 1,
            ore: 2,
            ..Default::default()
        },
        chains_to: vec![Card::Circus],
        colour: Colour::Red,
        power: Power::Shields(2),
    };
    static ref STABLES: CardInfo<'static> = CardInfo {
        name: "Stables",
        age: Age::Second,
        players_needed: vec![3, 5],
        cost: Cost {
            ore: 1,
            clay: 1,
            wood: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(2),
    };
    static ref ARCHERY_RANGE: CardInfo<'static> = CardInfo {
        name: "Archery Range",
        age: Age::Second,
        players_needed: vec![3, 6],
        cost: Cost {
            wood: 2,
            ore: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(2),
    };
}

lazy_static! {
    static ref PANTHEON: CardInfo<'static> = CardInfo {
        name: "Pantheon",
        age: Age::Third,
        players_needed: vec![3, 6],
        cost: Cost {
            clay: 2,
            ore: 1,
            papyrus: 1,
            loom: 1,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(7),
    };
    static ref GARDENS: CardInfo<'static> = CardInfo {
        name: "Gardens",
        age: Age::Third,
        players_needed: vec![3, 4],
        cost: Cost {
            wood: 2,
            clay: 2,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(5),
    };
    static ref TOWN_HALL: CardInfo<'static> = CardInfo {
        name: "Town Hall",
        age: Age::Third,
        players_needed: vec![3, 5, 6],
        cost: Cost {
            glass: 1,
            ore: 1,
            stone: 2,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(6),
    };
    static ref PALACE: CardInfo<'static> = CardInfo {
        name: "Palace",
        age: Age::Third,
        players_needed: vec![3, 7],
        cost: Cost {
            glass: 1,
            papyrus: 1,
            loom: 1,
            clay: 1,
            wood: 1,
            ore: 1,
            stone: 1,
            coins: 0,
        },
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(8),
    };
    static ref SENATE: CardInfo<'static> = CardInfo {
        name: "Senate",
        age: Age::Third,
        players_needed: vec![3, 5],
        cost: Cost {
            ore: 1,
            stone: 1,
            wood: 2,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Blue,
        power: Power::VictoryPoints(6),
    };
    static ref HAVEN: CardInfo<'static> = CardInfo {
        name: "Haven",
        age: Age::Third,
        players_needed: vec![3, 4],
        cost: Cost {
            loom: 1,
            ore: 1,
            wood: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::per_card_reward(Colour::Brown, true, false, 1, 1),
    };
    static ref LIGHTHOUSE: CardInfo<'static> = CardInfo {
        name: "Lighthouse",
        age: Age::Third,
        players_needed: vec![3, 6],
        cost: Cost {
            glass: 1,
            stone: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::per_card_reward(Colour::Yellow, true, false, 1, 1),
    };
    static ref CHAMBER_OF_COMMERCE: CardInfo<'static> = CardInfo {
        name: "Chamber Of Commerce",
        age: Age::Third,
        players_needed: vec![4, 6],
        cost: Cost {
            clay: 2,
            papyrus: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::per_card_reward(Colour::Grey, true, false, 2, 2),
    };
    static ref ARENA: CardInfo<'static> = CardInfo {
        name: "Arena",
        age: Age::Third,
        players_needed: vec![3, 5, 7],
        cost: Cost {
            ore: 1,
            stone: 2,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Yellow,
        power: Power::PerGameItemRewards(vec![PerGameItemReward {
            game_item: Box::new(|game_item| matches!(game_item, CountableGameItem::CompletedWonderStage)),
            me: true,
            neighbours: false,
            coins_per_thing: 3,
            points_per_thing: 1,
        }]),
    };
    static ref LODGE: CardInfo<'static> = CardInfo {
        name: "Lodge",
        age: Age::Third,
        players_needed: vec![3, 6],
        cost: Cost {
            clay: 2,
            loom: 1,
            papyrus: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Compass]),
    };
    static ref OBSERVATORY: CardInfo<'static> = CardInfo {
        name: "Observatory",
        age: Age::Third,
        players_needed: vec![3, 7],
        cost: Cost {
            ore: 2,
            glass: 1,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Cog]),
    };
    static ref UNIVERSITY: CardInfo<'static> = CardInfo {
        name: "University",
        age: Age::Third,
        players_needed: vec![3, 4],
        cost: Cost {
            wood: 2,
            papyrus: 1,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Tablet]),
    };
    static ref ACADEMY: CardInfo<'static> = CardInfo {
        name: "Academy",
        age: Age::Third,
        players_needed: vec![3, 7],
        cost: Cost {
            stone: 3,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Compass]),
    };
    static ref STUDY: CardInfo<'static> = CardInfo {
        name: "Study",
        age: Age::Third,
        players_needed: vec![3, 5],
        cost: Cost {
            wood: 1,
            papyrus: 1,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Green,
        power: Power::Science(vec![ScienceItem::Cog]),
    };
    static ref FORTIFICATIONS: CardInfo<'static> = CardInfo {
        name: "Fortifications",
        age: Age::Third,
        players_needed: vec![3, 7],
        cost: Cost {
            stone: 1,
            ore: 3,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(3),
    };
    static ref CIRCUS: CardInfo<'static> = CardInfo {
        name: "Circus",
        age: Age::Third,
        players_needed: vec![4, 5, 6],
        cost: Cost {
            stone: 3,
            ore: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(3),
    };
    static ref ARSENAL: CardInfo<'static> = CardInfo {
        name: "Arsenal",
        age: Age::Third,
        players_needed: vec![3, 4, 7],
        cost: Cost {
            ore: 1,
            wood: 2,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(3),
    };
    static ref SIEGE_WORKSHOP: CardInfo<'static> = CardInfo {
        name: "Siege Workshop",
        age: Age::Third,
        players_needed: vec![3, 5],
        cost: Cost {
            wood: 1,
            clay: 3,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Red,
        power: Power::Shields(3),
    };
    static ref WORKERS_GUILD: CardInfo<'static> = CardInfo {
        name: "Workers Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            ore: 2,
            clay: 1,
            stone: 1,
            wood: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::per_card_reward(Colour::Brown, false, true, 0, 1),
    };
    static ref CRAFTSMENS_GUILD: CardInfo<'static> = CardInfo {
        name: "Craftsmens Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            ore: 2,
            stone: 2,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::per_card_reward(Colour::Grey, false, true, 0, 2),
    };
    static ref TRADERS_GUILD: CardInfo<'static> = CardInfo {
        name: "Traders Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            loom: 1,
            papyrus: 1,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::per_card_reward(Colour::Yellow, false, true, 0, 1),
    };
    static ref PHILOSOPHERS_GUILD: CardInfo<'static> = CardInfo {
        name: "Philosophers Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            clay: 3,
            loom: 1,
            papyrus: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::per_card_reward(Colour::Green, false, true, 0, 1),
    };
    static ref SPIES_GUILD: CardInfo<'static> = CardInfo {
        name: "Spies Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            clay: 3,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::per_card_reward(Colour::Red, false, true, 0, 2),
    };
    static ref STRATEGISTS_GUILD: CardInfo<'static> = CardInfo {
        name: "Strategists Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            ore: 2,
            stone: 1,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::PerGameItemRewards(vec![PerGameItemReward {
            game_item: Box::new(|game_item| matches!(game_item, CountableGameItem::DefeatToken)),
            me: false,
            neighbours: true,
            coins_per_thing: 0,
            points_per_thing: 1,
        }]),
    };
    static ref SHIPOWNERS_GUILD: CardInfo<'static> = CardInfo {
        name: "Shipowners Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            wood: 3,
            papyrus: 1,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::PerGameItemRewards(vec![PerGameItemReward {
            game_item: Box::new(|game_item| {
                matches!(game_item,
                CountableGameItem::CountableCard(card) if
                    card.info().colour == Colour::Brown ||
                    card.info().colour == Colour::Grey ||
                    card.info().colour == Colour::Purple)
            }),
            me: true,
            neighbours: false,
            coins_per_thing: 0,
            points_per_thing: 1,
        }]),
    };
    static ref SCIENTISTS_GUILD: CardInfo<'static> = CardInfo {
        name: "Scientists Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            wood: 2,
            ore: 2,
            papyrus: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::Science(vec![ScienceItem::Compass, ScienceItem::Cog, ScienceItem::Tablet]),
    };
    static ref MAGISTRATES_GUILD: CardInfo<'static> = CardInfo {
        name: "Migistrates Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            wood: 3,
            stone: 1,
            loom: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::per_card_reward(Colour::Blue, false, true, 0, 1),
    };
    static ref BUILDERS_GUILD: CardInfo<'static> = CardInfo {
        name: "Builders Guild",
        age: Age::Third,
        players_needed: vec![3],
        cost: Cost {
            stone: 2,
            clay: 2,
            glass: 1,
            ..Default::default()
        },
        chains_to: vec![],
        colour: Colour::Purple,
        power: Power::PerGameItemRewards(vec![PerGameItemReward {
            game_item: Box::new(|game_item| matches!(game_item, CountableGameItem::CompletedWonderStage)),
            me: true,
            neighbours: true,
            coins_per_thing: 0,
            points_per_thing: 1,
        }]),
    };
}

#[allow(dead_code)]
impl Card {
    fn info(&self) -> &CardInfo {
        match self {
            Card::LumberYard => &LUMBER_YARD,
            Card::StonePit => &STONE_PIT,
            Card::ClayPool => &CLAY_POOL,
            Card::OreVein => &ORE_VEIN,
            Card::TreeFarm => &TREE_FARM,
            Card::Excavation => &EXCAVATION,
            Card::ClayPit => &CLAY_PIT,
            Card::TimberYard => &TIMBER_YARD,
            Card::ForestCave => &FOREST_CAVE,
            Card::Mine => &MINE,
            Card::Loom1 => &LOOM1,
            Card::Glassworks1 => &GLASSWORKS1,
            Card::Press1 => &PRESS1,
            Card::Pawnshop => &PAWNSHOP,
            Card::Baths => &BATHS,
            Card::Altar => &ALTAR,
            Card::Theater => &THEATER,
            Card::Tavern => &TAVERN,
            Card::EastTradingPost => &EAST_TRADING_POST,
            Card::WestTradingPost => &WEST_TRADING_POST,
            Card::Marketplace => &MARKETPLACE,
            Card::Apothecary => &APOTHECARY,
            Card::Workshop => &WORKSHOP,
            Card::Scriptorium => &SCRIPTORIUM,
            Card::Stockade => &STOCKADE,
            Card::Barracks => &BARRACKS,
            Card::GuardTower => &GUARD_TOWER,
            Card::Sawmill => &SAWMILL,
            Card::Quarry => &QUARRY,
            Card::Brickyard => &BRICKYARD,
            Card::Foundry => &FOUNDRY,
            Card::Loom2 => &LOOM2,
            Card::Glassworks2 => &GLASSWORKS2,
            Card::Press2 => &PRESS2,
            Card::Aqueduct => &AQUEDUCT,
            Card::Temple => &TEMPLE,
            Card::Statue => &STATUE,
            Card::Courthouse => &COURTHOUSE,
            Card::Forum => &FORUM,
            Card::Caravansery => &CARAVANSERY,
            Card::Vineyard => &VINEYARD,
            Card::Bazar => &BAZAR,
            Card::Dispensary => &DISPENSARY,
            Card::Laboratory => &LABORATORY,
            Card::Library => &LIBRARY,
            Card::School => &SCHOOL,
            Card::Walls => &WALLS,
            Card::TrainingGround => &TRAINING_GROUND,
            Card::Stables => &STABLES,
            Card::ArcheryRange => &ARCHERY_RANGE,
            Card::Pantheon => &PANTHEON,
            Card::Gardens => &GARDENS,
            Card::TownHall => &TOWN_HALL,
            Card::Palace => &PALACE,
            Card::Senate => &SENATE,
            Card::Haven => &HAVEN,
            Card::Lighthouse => &LIGHTHOUSE,
            Card::ChamberOfCommerce => &CHAMBER_OF_COMMERCE,
            Card::Arena => &ARENA,
            Card::Lodge => &LODGE,
            Card::Observatory => &OBSERVATORY,
            Card::University => &UNIVERSITY,
            Card::Academy => &ACADEMY,
            Card::Study => &STUDY,
            Card::Fortifications => &FORTIFICATIONS,
            Card::Circus => &CIRCUS,
            Card::Arsenal => &ARSENAL,
            Card::SiegeWorkshop => &SIEGE_WORKSHOP,
            Card::WorkersGuild => &WORKERS_GUILD,
            Card::CraftsmensGuild => &CRAFTSMENS_GUILD,
            Card::TradersGuild => &TRADERS_GUILD,
            Card::PhilosophersGuild => &PHILOSOPHERS_GUILD,
            Card::SpiesGuild => &SPIES_GUILD,
            Card::StrategistsGuild => &STRATEGISTS_GUILD,
            Card::ShipownersGuild => &SHIPOWNERS_GUILD,
            Card::ScientistsGuild => &SCIENTISTS_GUILD,
            Card::MagistratesGuild => &MAGISTRATES_GUILD,
            Card::BuildersGuild => &BUILDERS_GUILD,
        }
    }

    pub fn age(&self) -> &Age {
        &self.info().age
    }

    pub fn players_needed(&self) -> &Vec<u32> {
        &self.info().players_needed
    }

    pub fn cost(&self) -> &Cost {
        &self.info().cost
    }

    pub fn chains_to(&self) -> &Vec<Card> {
        &self.info().chains_to
    }

    pub fn colour(&self) -> &Colour {
        &self.info().colour
    }

    pub fn power(&self) -> &Power {
        &self.info().power
    }

    // returns the immediate strength
    pub fn immediate_strength(&self) -> f32 {
        match self.power() {
            Power::VictoryPoints(points) => *points as f32,
            _ => 0.0,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info().name)
    }
}

/// Creates a new, shuffled deck for the given age and number of players.
pub fn new_deck(age: &Age, player_count: u32) -> Vec<Card> {
    new_deck_without(age, player_count, &HashMap::new())
}

/// Creates a new, shuffled deck for the given age, with the cards in `missing` excluded. `missing` is a hash map from
/// card to the number of instances of that card that should be excluded. This is intended for playing algorithms that
/// want to allocate random cards to players (because they don't know the actual cards those players have in their
/// hands), but they know certain cards are definitely not part of those players hands, because they're in the
/// algorithm's hand or on the table.
pub fn new_deck_without(age: &Age, player_count: u32, missing: &HashMap<Card, u32>) -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    let mut guilds: Vec<Card> = vec![];

    // Add all cards with the correct age and number of players needed, added guilds to a separate vector for the time
    // being.
    for card in Card::iter() {
        if card.age() == age {
            let num_cards = card.players_needed().iter().filter(|i| *i <= &player_count).count() as u32;
            for _ in 0..(num_cards - missing.get(&card).unwrap_or(&0)) {
                if card.colour() == &Colour::Purple {
                    guilds.push(card);
                } else {
                    deck.push(card);
                }
            }
        }
    }

    let missing_guild_count = missing.keys().filter(|card| card.colour() == &Colour::Purple).count();
    let guild_count = (player_count + 2) - missing_guild_count as u32;

    // Shuffle the guilds separately and add player_count + 2 random ones to the deck.
    if *age == Age::Third {
        guilds.shuffle(&mut thread_rng());
        for _ in 0..guild_count {
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
    use std::iter::FromIterator;

    #[test]
    fn new_deck_has_right_number_of_cards() {
        assert_eq!(21, new_deck(&Age::First, 3).len());
        assert_eq!(28, new_deck(&Age::First, 4).len());
        assert_eq!(35, new_deck(&Age::First, 5).len());
        assert_eq!(42, new_deck(&Age::First, 6).len());
        assert_eq!(49, new_deck(&Age::First, 7).len());

        assert_eq!(21, new_deck(&Age::Second, 3).len());
        assert_eq!(28, new_deck(&Age::Second, 4).len());
        assert_eq!(35, new_deck(&Age::Second, 5).len());
        assert_eq!(42, new_deck(&Age::Second, 6).len());
        assert_eq!(49, new_deck(&Age::Second, 7).len());

        assert_eq!(21, new_deck(&Age::Third, 3).len());
        assert_eq!(28, new_deck(&Age::Third, 4).len());
        assert_eq!(35, new_deck(&Age::Third, 5).len());
        assert_eq!(42, new_deck(&Age::Third, 6).len());
        assert_eq!(49, new_deck(&Age::Third, 7).len());
    }

    #[test]
    fn no_second_or_third_age_cards_in_first_age_deck() {
        assert!(!new_deck(&Age::First, 3).contains(&Card::Sawmill));
        assert!(!new_deck(&Age::First, 3).contains(&Card::Pantheon));
    }

    #[test]
    fn new_deck_without_excludes_given_cards() {
        let deck = new_deck_without(&Age::First, 7, &HashMap::from_iter(vec![(Card::Tavern, 2)]));
        assert_eq!(49 - 2, deck.len());
        assert_eq!(1, deck.iter().filter(|card| **card == Card::Tavern).count());
    }
}
