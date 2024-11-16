use bytes::BytesMut;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct ChunkLoader {
    socket: Arc<Mutex<TcpStream>>,
    chunk_path: String,
}

impl ChunkLoader {
    pub fn new(socket: Arc<Mutex<TcpStream>>, chunk_path: String) -> Self {
        ChunkLoader { socket, chunk_path }
    }

    pub async fn load_chunk(&mut self, x: i32, z: i32) -> Result<(), String> {
        let mut file = tokio::fs::File::open(&self.chunk_path)
            .await
            .map_err(|e| format!("Failed to open chunk file: {}", e))?;

        let mut chunk_data = Vec::new();
        file.read_to_end(&mut chunk_data)
            .await
            .map_err(|e| format!("Failed to read chunk data: {}", e))?;

        let mut chunk: crate::packet::manager::PacketManager =
            crate::packet::manager::PacketManager::new(BytesMut::from(&chunk_data[..]), 0);
        let mut chunk_data_packet = crate::packet::manager::PacketManager::new(BytesMut::new(), 0);

        chunk_data_packet.append(&BytesMut::from(&(x as i32).to_be_bytes()[..]));
        chunk_data_packet.append(&BytesMut::from(&(z as i32).to_be_bytes()[..]));

        chunk.read_var_int();
        chunk.read_var_int();
        chunk.read_int();
        chunk.read_int();

        let mut socket_guard = self.socket.lock().await;
        socket_guard
            .write_all(&chunk_data_packet.build_packet(0x20))
            .await
            .map_err(|e| format!("Failed to send chunk data packet: {}", e))?;

        Ok(())
    }
}
