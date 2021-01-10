use crate::game::Game;

mod card;
mod wonder;
mod power;
mod resources;
mod player;
mod game;
mod utils;
mod table;

fn main() {
    let mut game = Game::new(7);
    loop {
        println!();
        let action = game.ask_for_action(0);
        println!("Selected action: {}", action.to_string());
        game.do_action(0, &action);
    }
}