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
}

// Singleton config
pub fn get_or_init_config(args: Option<Vec<String>>) -> &'static Config {
    CONFIG.get_or_init(|| {
        if let Some(a) = args {
            
            let mut config_toml_path: &String = &String::from("Config.toml");
            if a.len() > 1 {
                config_toml_path = &a[1];
            } 

            let toml_string = fs::read_to_string(config_toml_path).expect("Should have been able to read the file");

            let config: Config = toml::from_str(toml_string.as_str()).unwrap();
            return config
        } else {
            return Config {
                spells: vec![]
            }
        }

    })
}
