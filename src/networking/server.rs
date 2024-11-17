use crate::config::Config;
use log::info;
use tokio::net::TcpListener;

pub async fn start_server(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            pandoranet::handle_connection(socket).await.unwrap();
        });
    }
}
