use crate::card::Card;
use crate::player::PlayerBoard;
use crate::wonder::{WonderType, WonderSide};

mod card;
mod wonder;
mod power;
mod resources;
mod player;

fn main() {
    let card = Card::Baths;
    println!("Baths cost {:?}", card.cost());
    let player = PlayerBoard::new(WonderType::ColossusOfRhodes, WonderSide::A);
    println!("Can play Tree Farm? {}", player.can_play(Card::TreeFarm));
}
