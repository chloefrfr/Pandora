use std::sync::Arc;

use bytes::BytesMut;
use log::error;
use serde_json::json;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::packet::manager::PacketManager;

pub async fn send_status_response(
    socket: &Arc<Mutex<TcpStream>>,
    proto_version: i32,
    max_players: u32,
) {
    let response_data = json!({
        "version": {
            "name": "1.21.3",
            "protocol": proto_version,
        },
        "players": {
            "max": max_players,
            "online": 0,
        },
    });

    let mut response = PacketManager::new(BytesMut::new(), 0);
    response.write_string(&serde_json::to_string(&response_data).unwrap());

    if let Err(err) = socket
        .lock()
        .await
        .write_all(&response.build_packet(0x00))
        .await
    {
        error!("Failed to send status response: {}", err);
    }
}
