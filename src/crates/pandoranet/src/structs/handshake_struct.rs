use std::fmt::Display;

use log::debug;
use packet_manager::types::varint_types::VarInt;
use pandora_macros::Decode;
use pandora_utils::types::decode_types::Decode;
use serde_json::json;
use tokio::io::AsyncRead;
use tokio::io::AsyncSeek;

use crate::structs::protocol::ClientHandshakeRequest;
use crate::Connection;

#[derive(Decode, Debug)]
pub struct HandshakePacket {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}

impl Display for HandshakePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
                f,
                "HandshakePacket {{ protocol_version: {}, server_address: {}, server_port: {}, next_state: {} }}",
                self.protocol_version, self.server_address, self.server_port, self.next_state
            )
    }
}

impl HandshakePacket {
    pub async fn handle(self, conn: &mut Connection) -> Result<(), String> {
        debug!("Handshake packet received");

        let response_data = json!({
            "version": {
                "name": "1.16.5",
                "protocol": 754,
            },
            "players": {
                "max": 10,
                "online": 0,
            },
            "description": {
                "text": "A Minecraft server",
            },
        });

        let packet = ClientHandshakeRequest {
            packet_id: VarInt::new(0x00),
            res_json: response_data.to_string(),
        };

        let data = packet
            .encode()
            .await
            .map_err(|err| {
                log::error!("Failed to encode handshake packet {:?}", err);
            })
            .unwrap();

        conn.push_to_queue(data).await;

        Ok(())
    }
}
