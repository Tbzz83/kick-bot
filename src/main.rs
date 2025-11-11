mod config;
mod game;

use std::env;
use config::get_or_init_config;
use game::controller::GameController;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_or_init_config(Some(args));
    let mut game = GameController::new(config);
    if let Err(e) = game.run() {
        eprintln!("Error: {:?}", e);
    }
}

