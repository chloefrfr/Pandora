use log::{error, info};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_players: u32,
    pub motd: String,
}

impl Config {
    pub fn load_config() -> Self {
        let start_time = Instant::now();
        let config_path = Path::new("config.json");

        let config_str = fs::read_to_string(config_path).unwrap_or_else(|err| {
            error!("Failed to read config file: {}", err);
            panic!("Configuration file missing or malformed");
        });

        let config = serde_json::from_str(&config_str).unwrap_or_else(|err| {
            error!("Failed to parse config file: {}", err);
            panic!("Configuration file malformed");
        });

        let duration = start_time.elapsed();
        info!("Config::load_config completed in {:?}", duration);
        config
    }
}
