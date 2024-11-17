use crate::packet::manager::PacketManager;
use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_pong(
    buffer: &mut BytesMut,
    socket: &mut tokio::sync::MutexGuard<'_, TcpStream>,
) {
    let mut packet = PacketManager::new(buffer.clone(), 0);
    let mut pong = PacketManager::new(BytesMut::new(), 0);
    pong.write_long(packet.read_long().into());

    socket.write_all(&pong.build_packet(0x01)).await.unwrap();
}
