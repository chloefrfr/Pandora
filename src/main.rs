use log::{error, info};
use networking::server::start_server;

use std::time::Instant;

pub mod chunks;
pub mod config;
pub mod constants;
pub mod logger;
pub mod networking;
pub mod packets;

#[tokio::main]
async fn main() {
    logger::init_logger();

    info!("Starting Pandora");

    let start_time = Instant::now();
    let config = config::Config::load_config();
    let duration = start_time.elapsed();
    info!("Config loaded in {:?}", duration);

    if let Err(e) = start_server(&config).await {
        error!("Failed to start server: {}", e);
    }
}
