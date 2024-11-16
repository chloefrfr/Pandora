use log::{error, info};
use networking::server::start_server;
use simple_logger::SimpleLogger;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub mod config;
pub mod logger;
pub mod networking;
pub mod packet;
pub mod packets;

#[tokio::main]
async fn main() {
    logger::init_logger();

    info!("Starting Pandora");

    let config = config::Config::load_config();

    if let Err(e) = start_server(&config).await {
        error!("Failed to start server: {}", e);
    }
}
