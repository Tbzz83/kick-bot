use crate::config::{Config, Spell};

#[derive(Debug)]
pub struct GameState<'a> {
    pub config: &'a Config,
    pub current_target: Option<u16>,
    pub current_spell: Option<&'a Spell>,
}

impl<'a> GameState<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            current_target: None,
            current_spell: None,
        }
    }
}

