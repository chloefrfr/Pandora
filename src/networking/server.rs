use std::sync::Arc;

use crate::config::Config;
use crate::networking::connection::handle_connection;
use log::info;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn start_server(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    let connections = Arc::new(Mutex::new(Vec::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let connections = Arc::clone(&connections);

        tokio::spawn(async move {
            handle_connection(socket, connections).await;
        });
    }
}
