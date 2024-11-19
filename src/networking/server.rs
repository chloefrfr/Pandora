use log::info;
use tokio::net::TcpListener;

use crate::config::Config;

pub async fn start_server(config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        pandoranet::handle_connection(socket).await.unwrap();
    }
}
