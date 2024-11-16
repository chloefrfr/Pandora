use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,

    pub max_players: u32,
}

impl Config {
    pub fn load_config() -> Self {
        let config_path = Path::new("config.json");

        let config_str = fs::read_to_string(config_path).unwrap_or_else(|err| {
            error!("Failed to read config file: {}", err);
            panic!("Configuration file missing or malformed");
        });

        serde_json::from_str(&config_str).unwrap_or_else(|err| {
            error!("Failed to parse config file: {}", err);
            panic!("Configuration file malformed");
        })
    }
}
