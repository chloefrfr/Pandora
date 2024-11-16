use crate::packet::manager::{PacketHandler, PacketManager};
use crate::utils::send_status_response::send_status_response;
use log::info;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct HandshakePacket;

impl HandshakePacket {
    pub fn new() -> Self {
        HandshakePacket
    }
}

#[async_trait::async_trait]
impl PacketHandler for HandshakePacket {
    async fn handle(
        &self,
        packet: &mut PacketManager,
        socket: Arc<Mutex<TcpStream>>,
        state: &mut i32,
    ) -> Result<(), String> {
        let _protocol_version = packet.read_var_int();
        let _server_address = packet.read_string();
        let _server_port = packet.read_unsigned_short();
        let next_state = packet.read_var_int();

        *state = next_state;
        info!("State updated to: {}", state);

        if *state == 1 {
            tokio::spawn(async move {
                send_status_response(&socket, 10).await;
            });
        }

        Ok(())
    }
}
