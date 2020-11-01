use crate::card::Card;

mod card;
mod wonder;
mod power;
mod resources;
mod player;

fn main() {
    let card = Card::Baths;
    println!("Baths cost {:?}", card.cost());
}
