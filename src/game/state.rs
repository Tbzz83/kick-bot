use std::time::Instant;

use crate::config::{Config, Spell};

#[derive(Debug)]
pub struct GameState<'a> {
    pub config: &'a Config,
    pub current_target: Option<u16>,
    pub current_spell: Option<&'a Spell>,

    // Time elapsed since interruptable cast began in milliseconds
    pub spell_cast_time_elapsed_millis: Option<u128>,
}

impl<'a> GameState<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            current_target: None,
            current_spell: None,
            spell_cast_time_elapsed_millis: None,
        }
    }
}

