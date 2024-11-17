use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex};

use crate::PacketManager;

#[async_trait::async_trait]
pub trait PacketHandler {
    async fn handle(
        &self,
        packet: &mut PacketManager,
        socket: Arc<Mutex<TcpStream>>,
        state: &mut i32,
    ) -> Result<(), String>;
}
