use std::sync::Arc;

use bytes::BytesMut;
use tokio::{io::AsyncWriteExt, sync::Mutex};

pub async fn send_success_response(
    socket: Arc<Mutex<tokio::net::TcpStream>>,
    packet: &mut crate::packet::manager::PacketManager,
) -> Result<(), String> {
    let mut success = crate::packet::manager::PacketManager::new(BytesMut::new(), 0);
    success.write_uuid("550e8400-e29b-41d4-a716-446655440000");
    success.write_string(&packet.read_string());
    socket
        .lock()
        .await
        .write_all(&success.build_packet(0x02))
        .await
        .unwrap();

    Ok(())
}
