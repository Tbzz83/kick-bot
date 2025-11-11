use std::{fs, sync::OnceLock};
use serde::Deserialize;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub spells: Vec<Spell>,
}

#[derive(Debug, Deserialize)]
pub struct Spell {
    pub spell_name: String,
    pub keybinds: Vec<String>,
    pub spell_type: SpellType,
}

#[derive(Debug, Deserialize)]
pub enum SpellType {
    Interrupt,
    CC,
}

pub fn get_or_init_config(args: Option<Vec<String>>) -> &'static Config {
    CONFIG.get_or_init(|| {
        let config_path = args
            .as_ref()
            .and_then(|a| a.get(1))
            .cloned()
            .unwrap_or_else(|| "Config.toml".to_string());

        let toml_string = fs::read_to_string(config_path).expect("Unable to read config file");
        toml::from_str(&toml_string).expect("Invalid config file format")
    })
}

