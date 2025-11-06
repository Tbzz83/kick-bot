mod game;
mod config;

use std::env;
use crate::{config::{get_or_init_config}, game::game_loop};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_or_init_config(Some(args));
    game_loop(config);
}
