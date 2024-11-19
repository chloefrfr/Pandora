use std::sync::Arc;

use bytes::BytesMut;
use packet_manager::PacketManager;
use tokio::io::AsyncWriteExt;

pub async fn send_player_position(
    socket: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>,
) -> Result<(), String> {
    let mut position_packet = PacketManager::new(BytesMut::new(), 0);
    position_packet.write_double(0.0);
    position_packet.write_double(64.0);
    position_packet.write_double(0.0);
    position_packet.write_float(0.0);
    position_packet.write_float(0.0);
    position_packet.write_byte(0);
    position_packet.write_var_int_checked(0);

    socket
        .lock()
        .await
        .write_all(&position_packet.build_packet(0x34))
        .await
        .unwrap();

    Ok(())
}
