mod game;
mod config;
mod tui;

use std::env;
use crate::{config::get_or_init_config, game::game_loop, tui::tui_init};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_or_init_config(Some(args));
    //game_loop(config);
    tui_init(config);
}
