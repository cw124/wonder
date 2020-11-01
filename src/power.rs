use crate::card::Card;
use crate::resources::ProducedResources;

pub enum Power {
    PurchasableProducer(ProducedResources),
    Producer(ProducedResources),
    VictoryPoints(u32),
    Coins(u32),
    BuyAntiClockwise,
    PerGameItemRewards(Vec<PerGameItemReward>),
}

pub struct PerGameItemReward {
    pub game_item: fn(game_item: CountableGameItem) -> bool,
    pub me: bool,
    pub neighbours: bool,
    pub coins_per_thing: u32,
    pub points_per_thing: u32
}

pub enum CountableGameItem {
    CountableCard(Card),
}