use console::Term;
use std::time::{Instant};
use rand::Rng;

pub const WAIT_TIME_MILLIS: u128 = 10000;

// TODO get these values from a function that reads keybinds.yaml
pub const USER_KICKS: [&str; 3] = ["7", "8", "9"];
pub const USER_SHEEP: [&str; 3] = ["shift+7", "shift+8", "shift+9"];

fn get_random_wait_time() -> u128 {
    // Return random u128 in range 0 -> 1000
    rand::rng().random_range(0..WAIT_TIME_MILLIS)
}

fn main() {
    let stdout = Term::buffered_stdout();

    'game_loop: loop {
        let wait_time: u128 = get_random_wait_time();

        #[cfg(debug_assertions)]
        println!("wait time is {wait_time} (ms)");

        let now = Instant::now();

        while now.elapsed().as_millis() != wait_time {} // wait
        
        println!("KICK!");

        if let Ok(character) = stdout.read_char() {
            match character {
                'w' => todo!("Up"),
                'a' => todo!("Left"),
                's' => todo!("Down"),
                'd' => todo!("Right"),
                _ => break 'game_loop,
            }
        }
    }
}
