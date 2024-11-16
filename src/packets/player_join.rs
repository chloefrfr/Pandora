use bytes::BytesMut;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

use crate::chunks::chunk_loader::ChunkLoader;
use crate::packet::manager::PacketHandler;
use crate::utils::send_player_position::send_player_position;
use crate::utils::send_success_response::send_success_response;

pub struct PlayerJoinPacket;

impl PlayerJoinPacket {
    pub fn new() -> Self {
        PlayerJoinPacket
    }
}

const DIMENSION_CODEC_PATH: &str = "./NBT/dimension_codec.nbt";
const CHUNK_PATH: &str = "./NBT/chunk.nbt";
const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(10);

#[async_trait::async_trait]
impl PacketHandler for PlayerJoinPacket {
    async fn handle(
        &self,
        packet: &mut crate::packet::manager::PacketManager,
        socket: Arc<Mutex<tokio::net::TcpStream>>,
        _state: &mut i32,
    ) -> Result<(), String> {
        let socket_clone = Arc::clone(&socket);

        send_success_response(socket_clone, packet).await.unwrap();

        let mut join_game = crate::packet::manager::PacketManager::new(BytesMut::new(), 0);
        join_game.write_int(0);
        join_game.write_boolean(true);
        join_game.write_unsigned_byte(1);
        join_game.write_byte(-1);
        join_game.write_var_int(3);
        join_game.write_string("minecraft:overworld");

        let val = crate::utils::read_file::read_file(DIMENSION_CODEC_PATH).unwrap();
        join_game.append_blob(&val);

        join_game.write_string("minecraft:overworld");
        join_game.write_long(1);
        join_game.write_long(10);
        join_game.write_long(10);
        join_game.write_boolean(false);
        join_game.write_boolean(true);
        join_game.write_boolean(false);
        join_game.write_boolean(true);

        {
            let mut socket_guard = socket.lock().await;
            socket_guard
                .write_all(&join_game.build_packet(0x24))
                .await
                .unwrap();
        }

        let mut chunk_loader = ChunkLoader::new(socket.clone(), CHUNK_PATH.to_string());

        let result = chunk_loader.load_chunk(0, 0).await;
        if let Err(e) = result {
            return Err(format!("Failed to load chunk: {}", e));
        }

        send_player_position(socket.clone()).await.unwrap();

        let mut interval = tokio::time::interval(KEEP_ALIVE_INTERVAL);
        loop {
            interval.tick().await;
            let mut keep_alive = crate::packet::manager::PacketManager::new(BytesMut::new(), 0);
            keep_alive.write_long(0);
            {
                let mut socket_guard = socket.lock().await;
                socket_guard
                    .write_all(&keep_alive.build_packet(0x1F))
                    .await
                    .unwrap();
            }
        }
    }
}
